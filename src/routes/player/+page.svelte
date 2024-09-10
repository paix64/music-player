<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { Shortcut } from "../../Shortcut.js";
    import Navigation from "$lib/components/Navigation.svelte";
    import { onMount } from "svelte";

    import {
        PlayIcon,
        SkipForwardIcon,
        SkipBackIcon,
        RepeatIcon,
        ShuffleIcon,
    } from "svelte-feather-icons";
    import {
        adjustVolume,
        seekPosition,
        getCurrentSongInfo,
        skipMusic,
        playPause,
        getQueue,
        playerNotPlaying,
        getSongPosition,
        getPlayerRepeat,
        togglePlayerRepeat,
        shuffleMusic,
        importCSS,
    } from "../../service";

    let api: CarouselAPI;
    let current = 0;

    $: if (api) {
        current = api.selectedScrollSnap() + 1;

        api.on("select", () => {
            current = api.selectedScrollSnap() + 1;
        });
    }
    let song = {
        title: "",
        artist: "",
        album: "",
        album_cover: "",
        genre: "",
        year: "",
        track: "",
        duration: 1.0,
        duration_display: "",
    };

    let cover_queue: string[] = [];
    let song_position = 0.0;
    let song_position_display = "0:00";

    async function getCurrentSong() {
        const song = {
            title: await getCurrentSongInfo("title"),
            artist: await getCurrentSongInfo("artist"),
            album: await getCurrentSongInfo("album"),
            album_cover: await getCurrentSongInfo("album_cover"),
            genre: await getCurrentSongInfo("genre"),
            year: await getCurrentSongInfo("year"),
            track: await getCurrentSongInfo("track"),
            duration: await getCurrentSongInfo("duration").then((duration) => {
                if ((duration as number) == 0) {
                    return 1.0;
                }
                return duration;
            }),
            duration_display: "",
        };
        song.duration_display = await displayDuration(song.duration);
        return song;
    }

    async function displayDuration(of: number) {
        const minutes = Math.floor(of / 60);
        const seconds = of % 60;
        return `${minutes}:${seconds.toString().padStart(2, "0")}`;
    }

    async function nextSong() {
        if (cover_queue.length == current) {
            await skipMusic(1 - cover_queue.length);
            api.scrollTo(0);
        } else {
            await skipMusic(1);
            api.scrollNext();
        }
        await updateCurrentSong();
    }

    async function updateCurrentSong() {
        song = await getCurrentSong();
        if (song.title !== "") {
            localStorage.setItem(SONG_CACHE_KEY, JSON.stringify(song));
        }
    }

    const COVER_CACHE_KEY = "cover_queue_cache";
    const SONG_CACHE_KEY = "song_cache";
    onMount(async () => {
        const cachedCovers = localStorage.getItem(COVER_CACHE_KEY);
        if (cachedCovers) {
            cover_queue = JSON.parse(cachedCovers);
        } else {
            try {
                cover_queue = await getQueue();
                localStorage.setItem(
                    COVER_CACHE_KEY,
                    JSON.stringify(cover_queue),
                );
            } catch (e) {
                console.error("Failed to fetch cover queue", e);
            }
        }

        const cachedSong = localStorage.getItem(SONG_CACHE_KEY);
        if (cachedSong) {
            song = JSON.parse(cachedSong);
        } else {
            try {
                await updateCurrentSong();
            } catch (e) {
                console.error("Failed to fetch song info", e);
            }
        }
    });

    async function updateSongPosition() {
        song_position = await getSongPosition();
        song_position_display = await displayDuration(song_position);
        if (song.duration == 0) {
            return;
        }
        if (
            song.duration - song_position < 1 &&
            (await playerNotPlaying()) &&
            !(await getPlayerRepeat())
        ) {
            await nextSong();
        } else if (
            song.duration - song_position < 1 &&
            !(await playerNotPlaying()) &&
            (await getPlayerRepeat())
        ) {
            await skipMusic(0);
        }
    }
    importCSS();
    setInterval(updateSongPosition, 500);
</script>

<div class="main player">
    <Navigation />
    <p class="title-album">{song.album}</p>
    <Carousel.Root
        bind:api
        class="cover-list"
        opts={{
            watchDrag: false,
        }}
    >
        <Carousel.Content>
            {#if cover_queue.length > 0}
                {#each cover_queue as cover}
                    <Carousel.Item>
                        <div class="cover-border">
                            <img
                                src={convertFileSrc(cover)}
                                alt="Album Cover"
                                class="cover"
                                loading="lazy"
                            />
                        </div>
                    </Carousel.Item>
                {/each}
            {:else}
                <img
                    src={"src/assets/placeholder.jpg"}
                    alt="Album Cover"
                    class="cover"
                    loading="lazy"
                />
            {/if}
        </Carousel.Content>
    </Carousel.Root>

    <div class="information">
        <p class="title">{song.title}</p>
        <p class="artist">{song.artist}</p>
        <hr class="seperator" />
        <Progress
            value={song_position}
            max={song.duration}
            class="progress-bar"
        />
        <!-- <Slider value={[song_position]} max={song_duration} class="mx-auto" /> -->
        <p class="duration float-right">
            {song.duration_display}
        </p>
        <p class="duration">{song_position_display}</p>
    </div>

    Slide {current} of {cover_queue.length}

    <div class="controls">
        <button
            class="left-side"
            use:Shortcut={{ alt: false, code: "KeyN" }}
            on:click={async () => {
                await shuffleMusic();
            }}
        >
            <ShuffleIcon size="50rem" />
        </button>
        <button
            class="left-side"
            use:Shortcut={{ alt: false, code: "KeyN" }}
            on:click={async () => {
                await skipMusic(-1);
                api.scrollPrev();
                await updateCurrentSong();
            }}
        >
            <SkipBackIcon size="50rem" />
        </button>
        <button
            class="middle"
            use:Shortcut={{ shift: false, code: "Space" }}
            on:click={async () => await playPause()}
        >
            <PlayIcon size="50rem" class="ml-1.5" />
        </button>
        <button
            class="right-side"
            use:Shortcut={{ control: false, code: "KeyM" }}
            on:click={async () => {
                await nextSong();
            }}
        >
            <SkipForwardIcon size="50em" />
        </button>
        <button
            class="right-side"
            use:Shortcut={{ control: false, code: "KeyR" }}
            on:click={async () => {
                await togglePlayerRepeat();
            }}
        >
            <RepeatIcon size="50em" />
        </button>
    </div>
    <div>
        <button
            use:Shortcut={{ code: "ArrowRight" }}
            on:click={async () => {
                await seekPosition(10);
            }}
        ></button>
        <button
            use:Shortcut={{ code: "ArrowLeft" }}
            on:click={async () => {
                await seekPosition(-10);
            }}
        ></button>
        <button
            use:Shortcut={{ code: "ArrowUp" }}
            on:click={async () => {
                await adjustVolume(0.05);
            }}
        ></button>
        <button
            use:Shortcut={{ code: "ArrowDown" }}
            on:click={async () => {
                await adjustVolume(-0.05);
            }}
        ></button>
    </div>
</div>

<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { Shortcut } from "../../Shortcut.js";
    import Navigation from "$lib/components/Navigation.svelte";
    import Shortcuts from "$lib/components/Shortcuts.svelte";
    import { onMount } from "svelte";

    import {
        PlayIcon,
        PauseIcon,
        SkipForwardIcon,
        SkipBackIcon,
        RepeatIcon,
        ShuffleIcon,
    } from "svelte-feather-icons";
    import {
        playerCurrentSongInfo,
        playerSkip,
        playerPlayOrPause,
        playerCoverPathQueue,
        playerSongFinished,
        playerSongPosition,
        playerRepeat,
        playerToggleRepeat,
        playerShuffleQueue,
        playerSongPaused,
        importCSS,
    } from "../../service";

    let api: CarouselAPI;
    let current = 0;
    let paused: boolean;
    let loading = true;

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
        cover_path: "",
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
            title: await playerCurrentSongInfo("title"),
            artist: await playerCurrentSongInfo("artist"),
            album: await playerCurrentSongInfo("album"),
            cover_path: await playerCurrentSongInfo("cover_path"),
            genre: await playerCurrentSongInfo("genre"),
            year: await playerCurrentSongInfo("year"),
            track: await playerCurrentSongInfo("track"),
            duration: await playerCurrentSongInfo("duration").then(
                (duration) => {
                    if ((duration as number) == 0) {
                        return 1.0;
                    }
                    return duration;
                },
            ),
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
            await playerSkip(1 - cover_queue.length);
            api.scrollTo(0);
        } else {
            await playerSkip(1);
            api.scrollNext();
        }
        await updateCurrentSong();
    }

    async function updateCurrentSong() {
        loading = true;
        song = await getCurrentSong();
        loading = false;
        if (song.title !== "") {
            localStorage.setItem(SONG_CACHE_KEY, JSON.stringify(song));
        }
    }

    const COVER_CACHE_KEY = "cover_queue_cache";
    const SONG_CACHE_KEY = "song_cache";
    onMount(async () => {
        paused = !(await playerSongPaused());

        const cachedCovers = localStorage.getItem(COVER_CACHE_KEY);
        if (cachedCovers) {
            cover_queue = JSON.parse(cachedCovers);
        } else {
            try {
                cover_queue = await playerCoverPathQueue();
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
        song_position = await playerSongPosition();
        song_position_display = await displayDuration(song_position);
        if (song.duration == 0) {
            return;
        }
        if (
            song.duration - song_position < 1 &&
            (await playerSongFinished()) &&
            !(await playerRepeat())
        ) {
            await nextSong();
        } else if (
            song.duration - song_position < 1 &&
            !(await playerSongFinished()) &&
            (await playerRepeat())
        ) {
            await playerSkip(0);
        }
    }
    importCSS();
    setInterval(updateSongPosition, 500);
</script>


{#if loading}
    <div>
        <Navigation />
    </div>
{:else}
<div class="main player">
    <Navigation />
    <Shortcuts />
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

    <div class="controls">
        <button
            class="left-side"
            use:Shortcut={{ alt: false, code: "KeyB" }}
            on:click={async () => {
                await playerShuffleQueue();
            }}
        >
            <ShuffleIcon size="50rem" />
        </button>
        <button
            class="left-side"
            use:Shortcut={{ alt: false, code: "KeyN" }}
            on:click={async () => {
                await playerSkip(-1);
                api.scrollPrev();
                await updateCurrentSong();
            }}
        >
            <SkipBackIcon size="50rem" />
        </button>
        <button
            class="middle"
            use:Shortcut={{ shift: false, code: "Space" }}
            on:click={async () => {
                await playerPlayOrPause();
                paused = !(await playerSongPaused());
            }}
        >
            {#if paused}
                <PauseIcon size="50rem" />
            {:else}
                <PlayIcon size="50rem" />
            {/if}
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
                await playerToggleRepeat();
            }}
        >
            <RepeatIcon size="50em" />
        </button>
    </div>
</div>
{/if}
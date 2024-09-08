<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { Shortcut } from "../Shortcut.js";
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
    } from "../service";

    let api: CarouselAPI;
    let current = 0;
    let count = 0;

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
        duration: 100.0,
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
                if (duration === 0) {
                    return 100.0;
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
        localStorage.setItem(SONG_CACHE_KEY, JSON.stringify(song));
    }

    const CACHE_KEY = "cover_queue_cache";
    const SONG_CACHE_KEY = "song_cache";
    onMount(async () => {
        const cachedCovers = localStorage.getItem(CACHE_KEY);
        const cachedSong = localStorage.getItem(SONG_CACHE_KEY);

        if (cachedCovers) {
            cover_queue = JSON.parse(cachedCovers);
        } else {
            try {
                cover_queue = await getQueue();
                localStorage.setItem(CACHE_KEY, JSON.stringify(cover_queue));
            } catch (e) {
                console.error("Failed to fetch cover queue", e);
            }
        }
        if (cachedSong) {
            song = JSON.parse(cachedSong);
        } else {
            try {
                song = await getCurrentSong();
                localStorage.setItem(SONG_CACHE_KEY, JSON.stringify(song));
            } catch (e) {
                console.error("Failed to fetch cover queue", e);
            }
        }
    });

    async function updateSongPosition() {
        song_position = await getSongPosition();
        song_position_display = await displayDuration(song_position);
        await getCurrentSong();

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

<div class="main">
    <Navigation />
    <p class="mt-[0.75%] text-xl opacity-70">{song.album}</p>
    <Carousel.Root
        bind:api
        class="my-[1.25%] mx-auto w-[80%] sm:w-[80%] md:w-[70%] lg:w-[60%] xl:w-[40%]"
        opts={{
            watchDrag: false,
        }}
    >
        <Carousel.Content>
            {#each cover_queue as _cover, i (i)}
                <Carousel.Item>
                    <div
                        class="p-0 border-2 rounded-3xl overflow-hidden border-slate-900"
                    >
                        <div class="">
                            <img
                                src={convertFileSrc(cover_queue[i])}
                                alt="Album Cover"
                                class="w-full h-full object-cover"
                                loading="lazy"
                            />
                        </div>
                    </div>
                </Carousel.Item>
            {/each}
        </Carousel.Content>
    </Carousel.Root>

    <div class="text-left w-[80%] mx-auto">
        <p class="text-3xl">{song.title}</p>
        <p class="text-xl opacity-70">{song.artist}</p>
        <hr class="my-2 border-t border-white" />
        <Progress value={song_position} max={song.duration} class="h-4" />
        <!-- <Slider value={[song_position]} max={song_duration} class="mx-auto" /> -->
        <p class="opacity-50 text-sm float-right mx-1">
            {song.duration_display}
        </p>
        <p class="opacity-50 text-sm mx-1">{song_position_display}</p>
    </div>

    <div class="text-muted-foreground py-2 text-center text-sm">
        Slide {current} of {count}
    </div>

    <div class="text-slate-600">
        <button
            class="my-4 mr-10 rounded-full shadow-2xl p-3"
            use:Shortcut={{ alt: false, code: "KeyN" }}
            on:click={async () => {
                await shuffleMusic();
            }}
        >
            <ShuffleIcon size="50rem" />
        </button>
        <button
            class="my-4 mr-10 rounded-full shadow-2xl p-3"
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
            class="my-4 rounded-full shadow-2xl p-3"
            use:Shortcut={{ shift: false, code: "Space" }}
            on:click={async () => await playPause()}
        >
            <PlayIcon size="50rem" class="ml-1.5" />
        </button>
        <button
            class="my-4 ml-10 rounded-full shadow-2xl p-3"
            use:Shortcut={{ control: false, code: "KeyM" }}
            on:click={async () => {
                await nextSong();
                await updateCurrentSong();
            }}
        >
            <SkipForwardIcon size="50em" />
        </button>
        <button
            class="my-4 ml-10 rounded-full shadow-2xl p-3"
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

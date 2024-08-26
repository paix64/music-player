<script lang="ts">
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { shortcut } from "../shortcut.js";
    import {
        PlayIcon,
        SkipForwardIcon,
        SkipBackIcon,
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
    } from "../service";
    import { appConfigDir } from "@tauri-apps/api/path";

    let api: CarouselAPI;
    let configDir: string;
    let current = 0;
    let count = 0;

    async function init() {
        configDir = await appConfigDir();        
    }

    $: if (api) {
        current = api.selectedScrollSnap() + 1;

        api.on("select", () => {
            current = api.selectedScrollSnap() + 1;
        });
    }

    let song_title = "";
    let song_artist = "";
    let song_album = "";
    let song_album_cover = "";
    let song_genre = "";
    let song_year = "";
    let song_track = "";
    let song_track_total = "";

    let cover_queue: string[] = [];

    let song_duration = 100.0;
    let song_position = 0.0;
    let song_duration_display = "0:00";
    let song_position_display = "0:00";

    async function getCurrentSong() {
        song_title = await getCurrentSongInfo("title");
        song_artist = await getCurrentSongInfo("artist");
        song_album = await getCurrentSongInfo("album");
        song_album_cover = await getCurrentSongInfo("album_cover");
        song_genre = await getCurrentSongInfo("genre");
        song_year = await getCurrentSongInfo("year");
        song_track = await getCurrentSongInfo("track");
        song_track_total = await getCurrentSongInfo("track_total");
        song_duration = await getCurrentSongInfo("duration").then(
            (duration) => {
                if ((duration as number) == 0) {
                    return 100.0;
                }
                return duration as number;
            },
        );
        song_duration_display = await displayDuration(song_duration);
    }

    async function displayDuration(of: number) {
        const minutes = Math.floor(of / 60);
        const seconds = of % 60;
        return `${minutes}:${seconds.toString().padStart(2, "0")}`;
    }

    async function updateSongPosition() {
        song_position = await getSongPosition();
        song_position_display = await displayDuration(song_position);
        cover_queue = await getQueue();
        await getCurrentSong();

        if (song_duration - song_position < 1 && (await playerNotPlaying())) {
            await skipMusic(1);
        }
    }
    init();
    setInterval(updateSongPosition, 500);
</script>

<div class="main non-selectable">
    <p class="mt-[0.75%] text-xl opacity-70">{song_album}</p>
    <Carousel.Root
        bind:api
        class="my-[1.25%] mx-auto w-[80%] sm:w-[80%] md:w-[70%] lg:w-[60%] xl:w-[40%]"
        opts={{
            watchDrag: false,
        }}
    >
        <Carousel.Content>
            {#each Array(cover_queue.length) as _, i (i)}
                <Carousel.Item>
                    <div
                        class="p-0 border-2 rounded-3xl overflow-hidden border-slate-900"
                    >
                        <div class="">
                            <img
                                src={convertFileSrc(cover_queue[i])}
                                alt="Album Cover"
                                class="w-full h-full object-cover"
                            />
                        </div>
                    </div>
                </Carousel.Item>
            {/each}
        </Carousel.Content>
    </Carousel.Root>

    <div class="text-left w-[80%] mx-auto">
        <p class="text-3xl">{song_title}</p>
        <p class="text-xl opacity-70">{song_artist}</p>
        <hr class="my-2 border-t border-white" />
        <Progress value={song_position} max={song_duration} class="h-4" />
        <!-- <Slider value={[song_position]} max={song_duration} class="mx-auto" /> -->
        <p class="opacity-50 text-sm float-right mx-1">
            {song_duration_display}
        </p>
        <p class="opacity-50 text-sm mx-1">{song_position_display}</p>
    </div>

    <div class="text-muted-foreground py-2 text-center text-sm">
        Slide {current} of {count}
    </div>

    <div class="text-slate-600">
        <button
            class="my-4 mr-10 rounded-full shadow-2xl p-3"
            use:shortcut={{ alt: false, code: "KeyN" }}
            on:click={async () => {
                await skipMusic(-1);
                api.scrollPrev();
            }}
        >
            <SkipBackIcon size="50rem" />
        </button>
        <button
            class="my-4 rounded-full shadow-2xl p-3"
            use:shortcut={{ shift: false, code: "Space" }}
            on:click={async () => await playPause()}
        >
            <PlayIcon size="50rem" class="ml-1.5" />
        </button>
        <button
            class="my-4 ml-10 rounded-full shadow-2xl p-3"
            use:shortcut={{ control: false, code: "KeyM" }}
            on:click={async () => {
                await skipMusic(1);
                api.scrollNext();
            }}
        >
            <SkipForwardIcon size="50em" />
        </button>
    </div>
    <div>
        <button
            use:shortcut={{ code: "ArrowRight" }}
            on:click={async () => {
                await seekPosition(10);
            }}
        ></button>
        <button
            use:shortcut={{ code: "ArrowLeft" }}
            on:click={async () => {
                await seekPosition(-10);
            }}
        ></button>
        <button
            use:shortcut={{ code: "ArrowUp" }}
            on:click={async () => {
                await adjustVolume(0.05);
            }}
        ></button>
        <button
            use:shortcut={{ code: "ArrowDown" }}
            on:click={async () => {
                await adjustVolume(-0.05);
            }}
        ></button>
    </div>
</div>

<style src="../theme.css"></style>

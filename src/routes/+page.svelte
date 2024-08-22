<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";
    import { Slider } from "$lib/components/ui/slider";
    import {
        PlayIcon,
        SkipForwardIcon,
        SkipBackIcon,
    } from "svelte-feather-icons";

    let api: CarouselAPI;
    let current = 0;
    let count = 0;

    $: if (api) {
        count = api.scrollSnapList().length;
        current = api.selectedScrollSnap() + 1;

        api.on("select", () => {
            current = api.selectedScrollSnap() + 1;
        });
    }

    let song_title = "";
    let song_artist = "";
    let song_album = "";
    let song_genre = "";
    let song_year = "";
    let song_track = "";
    let song_track_total = "";

    let song_duration = 100.0;
    let song_position = 0.0;
    let song_length_display = "0:00";
    let song_position_display = "0:00";

    async function playMusic() {
        await invoke("play_music");
        await getCurrentSong();
    }
    async function pauseResume() {
        await invoke("pause_resume");
    }
    async function skipMusic() {
        await invoke("skip_music");
        await getCurrentSong();
    }
    async function addMusic() {
        await invoke("add_music");
    }
    async function getCurrentSongInfo(key: string): Promise<any> {
        return await invoke("get_current_song_info", { key });
    }
    async function getCurrentSong() {
        song_title = await getCurrentSongInfo("title");
        song_artist = await getCurrentSongInfo("artist");
        song_album = await getCurrentSongInfo("album");
        song_genre = await getCurrentSongInfo("genre");
        song_year = await getCurrentSongInfo("year");
        song_track = await getCurrentSongInfo("track");
        song_track_total = await getCurrentSongInfo("track_total");
        song_duration = await getCurrentSongInfo("duration").then((dur) => {
            if ((dur as number) == 0) {
                return 100.0;
            }
            const minutes = Math.floor(dur / 60);
            const seconds = dur % 60;
            song_length_display = `${minutes}:${seconds.toString().padStart(2, "0")}`;

            return dur as number;
        });
    }

    async function seekMusic(pos: number) {
        await invoke("seek_position", { pos });
    }

    async function getSongPosition(): Promise<any> {
        song_position = await invoke("get_song_position");

        const minutes = Math.floor(song_position / 60);
        const seconds = song_position % 60;
        song_position_display = `${minutes}:${seconds.toString().padStart(2, "0")}`;
    }

    async function updateSongPosition() {
        await getSongPosition();
    }
    setInterval(updateSongPosition, 500);
</script>

<div class="container non-selectable">
    <Carousel.Root
        bind:api
        class="my-20 mx-auto w-[90%] sm:w-[80%] md:w-[70%] lg:w-[60%] xl:w-[50%]"
        opts={{
            loop: true,
        }}
    >
        <Carousel.Content>
            {#each Array(5) as _, i (i)}
                <Carousel.Item>
                    <div class="p-1">
                        <Card.Root>
                            <Card.Content
                                class="flex aspect-square items-center justify-center p-6"
                            >
                                <img src="static/favicon.png" alt="aa" />
                            </Card.Content>
                        </Card.Root>
                    </div>
                </Carousel.Item>
            {/each}
        </Carousel.Content>
        <!-- <Carousel.Previous />
        <Carousel.Next /> -->
    </Carousel.Root>

    <div class="text-left w-[80%] mx-auto">
        <p class="text-3xl">{song_title}</p>
        <p class="text-xl opacity-70">{song_artist}</p>
        <hr class="my-2 border-t border-white" />
        {song_position / song_duration}
        <Progress value={song_position} max={song_duration} class="h-4" />
        <!-- <Slider value={[song_position]} max={song_duration} class="mx-auto" /> -->
        <p class="opacity-50 text-sm float-right mx-1">{song_length_display}</p>
        <p class="opacity-50 text-sm mx-1">{song_position_display}</p>
    </div>

    <div class="text-muted-foreground py-2 text-center text-sm">
        Slide {current} of {count}
    </div>

    <div class="text-slate-400">
        <button
            class="my-4 mr-10 rounded-full bg-slate-200 p-3"
            on:click={async () => await skipMusic()}
        >
            <SkipBackIcon size="50rem" />
        </button>
        <button
            class="my-4 rounded-full bg-slate-200 p-3"
            on:click={async () => await playMusic()}
        >
            <PlayIcon size="50rem" />
        </button>
        <button
            class="my-4 ml-10 rounded-full bg-slate-200 p-3"
            on:click={async () => await skipMusic()}
        >
            <SkipForwardIcon size="50em" />
        </button>
        <!-- <button class="my-4" on:click={async () => await addMusic()}>add</button
        > -->
        <!-- <button class="my-4" on:click={async () => await pauseResume()}
            >pause</button
        > -->
    </div>
</div>

<style>
    .non-selectable {
        user-select: none;
    }

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 600;

        color: #0f0f0f;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>

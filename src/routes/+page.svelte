<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Carousel from "$lib/components/ui/carousel/index.js";
    import type { CarouselAPI } from "$lib/components/ui/carousel/context.js";
    import { Progress } from "$lib/components/ui/progress";

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

    let name = "";
    let greetMsg = "";

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsg = await invoke("greet", { name });
    }
</script>

<div class="container non-selectable">
    <Carousel.Root
        bind:api
        class="my-20 mx-96 "
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
                                <span class="text-4xl font-semibold"
                                    >{i + 1}</span
                                >
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
        <p class="text-3xl">Smells like teen spirit</p>
        <p class="text-xl opacity-70">Nirvana</p>
        <hr class="my-2 border-t border-white" />
        <Progress value={current * 20} class="h-4" />
    </div>

    <div class="text-muted-foreground py-2 text-center text-sm">
        Slide {current} of {count}
    </div>
    <!-- <form class="row" on:submit|preventDefault={greet}>
        <input
            id="greet-input"
            placeholder="Enter a name..."
            bind:value={name}
        />
        <button type="submit">Greet</button>
    </form>

    <p>{greetMsg}</p> -->
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

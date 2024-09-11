<script lang="ts">
    import { playerCurrentSongInfo, playerPlayOrPause } from "../../service";
    import { Shortcut } from "../../Shortcut.js";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let current_song_album_cover = "";
    let current_song_title = "";
    export async function getCurrentSong() {
        current_song_title = await playerCurrentSongInfo("title");
        current_song_album_cover = await playerCurrentSongInfo("cover_path");
    }
    getCurrentSong();
</script>

<div class="fixed left-0 bottom-0 m-0 h-20 w-screen shadow-inner flex bg-white">
    <button
        class="flex flex-row mx-auto my-auto"
        use:Shortcut={{ shift: false, code: "Space" }}
        on:click={() => playerPlayOrPause()}
    >
        {#if current_song_album_cover !== ""}
            <div class="border-2 rounded-xl overflow-hidden">
                <img
                    src={convertFileSrc(current_song_album_cover)}
                    alt="Album Cover"
                    class="size-16"
                    loading="lazy"
                />
            </div>
        {:else}
            <img
                src={"src/assets/placeholder.jpg"}
                alt="Album Cover"
                class="size-16"
                loading="lazy"
            />
        {/if}

        {#if current_song_title !== ""}
            <p class="ml-4 my-auto">{current_song_title}</p>
        {:else}
            <p class="ml-4 my-auto"></p>
        {/if}
    </button>
</div>

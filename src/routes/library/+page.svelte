<script lang="ts">
    import Navigation from "$lib/components/Navigation.svelte";
    import { onMount } from "svelte";
    import {
        getAlbumPlaylists,
        playAlbumPlaylist,
        importCSS,
    } from "../../service";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let albums: any[] = [];
    const CACHE_KEY = "albums_cache";
    onMount(async () => {
        const cachedAlbums = localStorage.getItem(CACHE_KEY);

        if (cachedAlbums) {
            albums = JSON.parse(cachedAlbums);
        } else {
            try {
                albums = await getAlbumPlaylists();
                localStorage.setItem(CACHE_KEY, JSON.stringify(albums));
            } catch (e) {
                console.error("Failed to fetch albums", e);
            }
        }
    });
    importCSS();
</script>

<div class="main">
    <Navigation />
    <p class="my-4">Albums</p>

    <div class="flex flex-row flex-wrap ml-12">
        {#each albums as _album, i (i)}
            <button
                on:click={() => {
                    localStorage.removeItem("song_cache");
                    localStorage.removeItem("cover_queue_cache");
                    playAlbumPlaylist(albums[i].name);
                }}
            >
                <div
                    class="p-0 m-4 border-2 rounded-3xl overflow-hidden border-slate-900 size-80"
                >
                    <img
                        src={convertFileSrc(albums[i].song_list[0].cover_path)}
                        alt="Album Cover"
                        class="h-full object-cover"
                        loading="lazy"
                    />
                </div>
                <p class="mb-4 text-2xl">{albums[i].name}</p>
            </button>
        {/each}
    </div>
</div>

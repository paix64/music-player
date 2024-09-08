<script lang="ts">
    import Navigation from "$lib/components/Navigation.svelte";
    import { onMount } from "svelte";
    import {
        getAlbumPlaylists,
        playAlbumPlaylist,
        importCSS,
    } from "../service";
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
    localStorage.removeItem("song_cache");
    localStorage.removeItem("cover_queue_cache");
</script>

<div class="main library">
    <Navigation />
    <p class="title-albums">Albums</p>

    <div class="playlist-list">
        {#each albums as album}
            <button
                on:click={() => {
                    localStorage.removeItem("song_cache");
                    localStorage.removeItem("cover_queue_cache");
                    playAlbumPlaylist(album.name);
                }}
            >
                <div class="cover-border">
                    <img
                        src={convertFileSrc(album.song_list[0].cover_path)}
                        alt="Album Cover"
                        class="cover"
                        loading="lazy"
                    />
                </div>
                <p class="album-name">{album.name}</p>
            </button>
        {/each}
    </div>
</div>

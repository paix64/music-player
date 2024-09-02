<script lang="ts">
    import Navigation from "$lib/components/Navigation.svelte";
    import { getAlbumPlaylists } from "../../service";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let albums: any[] = [];
    async function getAlbums() {
        albums = await getAlbumPlaylists();
    }

    getAlbums();
    console.log(albums);
</script>

<div class="main">
    <Navigation />
    <p class="my-4">Albums</p>

    <div class="flex flex-row flex-wrap ml-12">
        {#each Array(albums.length) as _, i (i)}
            <div
                class="p-0 m-4 border-2 rounded-3xl overflow-hidden border-slate-900 size-80"
            >
                <img
                    src={convertFileSrc(albums[i].song_list[0].cover_path)}
                    alt="Album Cover"
                />
            </div>
        {/each}
    </div>
</div>

<style src="../../theme.css"></style>

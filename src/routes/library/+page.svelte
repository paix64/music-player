<script>
    import Navigation from "$lib/components/Navigation.svelte";
    import { getAlbumPlaylists } from "../../service";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let albums = [];
    async function getAlbums() {
        albums = await getAlbumPlaylists();
    }

    getAlbums();
    console.log(albums);
</script>

<div class="main flex-col non-selectable">
    <Navigation />
    <p class="my-4">Albums</p>

    <div class="flex flex-row ml-8">
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

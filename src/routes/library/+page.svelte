<script lang="ts">
    import Navigation from "$lib/components/Navigation.svelte";
    import { getAlbumPlaylists, playAlbumPlaylist, importCSS} from "../../service";
    import { convertFileSrc } from "@tauri-apps/api/core";

    let albums: any[] = [];
    async function getAlbums() {
        albums = await getAlbumPlaylists();
    }

    getAlbums();
    console.log(albums);

    
    importCSS();
</script>

<div class="main">
    <Navigation />
    <p class="my-4">Albums</p>

    <div class="flex flex-row flex-wrap ml-12">
        {#each Array(albums.length) as _, i (i)}
            <button
                on:click={() => {
                    playAlbumPlaylist(albums[i].name);
                    console.log(albums[i].name);
                }}
            >
                <div
                    class="p-0 m-4 border-2 rounded-3xl overflow-hidden border-slate-900 size-80"
                >
                    <img
                        src={convertFileSrc(albums[i].song_list[0].cover_path)}
                        alt="Album Cover"
                        class="h-full object-cover"
                    />
                </div>
                <p class="mb-4 text-2xl">{albums[i].name}</p>
            </button>
        {/each}
    </div>
</div>

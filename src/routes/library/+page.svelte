<script>
    import Navigation from "$lib/components/Navigation.svelte";
    import { getAlbumPlaylist } from "../../service";

    getAlbumPlaylist("Nevermind").then((res) => {
        console.log(res);
    });

    getAlbumPlaylist("In Utero").then((res) => {
        console.log(res);
    });
</script>

<div class="main">
    <Navigation />
    <p>Albums</p>
    <div class="flex">
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
        <img class="mx-8 my-8" src="/favicon.png" alt="a" />
    </div>
</div>

<style>
    :root {
    }
    .main {
        margin-left: 5rem;
        margin-top: 2rem;
    }
    p {
        font-size: 2rem;
    }
</style>

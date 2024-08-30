<script>
    import {
        PlayCircleIcon,
        FolderIcon,
        SearchIcon,
    } from "svelte-feather-icons";
</script>

<div
    class="fixed top-0 left-0 h-screen w-16 rounded text-slate-700 shadow-xl m-0 flex flex-col"
>
    <div class="my-auto mx-2">
        <button
            class="my-2 w-12 h-12 hover:text-red-500 transition-all"
            on:click={() => (window.location.href = "/")}
        >
            <PlayCircleIcon size="50rem"></PlayCircleIcon>
        </button>

        <button
            class="my-2 w-12 h-12 hover:text-red-500 transition-all"
            on:click={() => (window.location.href = "/library")}
        >
            <FolderIcon size="50rem"></FolderIcon>
        </button>

        <button
            class="my-2 w-12 h-12 hover:text-red-500 transition-all"
            on:click={() => (window.location.href = "/library")}
        >
            <SearchIcon size="50rem"></SearchIcon>
        </button>
    </div>
</div>

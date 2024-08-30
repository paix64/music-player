<script>
    import Navigation from "$lib/components/Navigation.svelte";
    let message = "Hello, Svelte!";
</script>

<div>
    <Navigation />
    <h1>{message}</h1>
</div>

<style>
    :root {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }

    h1 {
        font-size: 2rem;
        color: #333;
    }
</style>
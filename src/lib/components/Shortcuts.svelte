<script>
    import { Shortcut } from "../../Shortcut.js";
    import { playerAdjustVolume, playerSeekPosition } from "../../service";
</script>

<div>
    <button
        use:Shortcut={{ code: "ArrowRight" }}
        on:click={async () => {
            await playerSeekPosition(10);
        }}
    ></button>
    <button
        use:Shortcut={{ code: "ArrowLeft" }}
        on:click={async () => {
            await playerSeekPosition(-10);
        }}
    ></button>
    <button
        use:Shortcut={{ code: "ArrowUp" }}
        on:click={async () => {
            await playerAdjustVolume(0.05);
        }}
    ></button>
    <button
        use:Shortcut={{ code: "ArrowDown" }}
        on:click={async () => {
            await playerAdjustVolume(-0.05);
        }}
    ></button>
    <button
        use:Shortcut={{ code: "Space" }}
        on:click={async () => {
            await playerAdjustVolume(-0.05);
        }}
    ></button>
    <button
        use:Shortcut={{ code: "ArrowDown" }}
        on:click={async () => {
            await playerAdjustVolume(-0.05);
        }}
    ></button>
</div>
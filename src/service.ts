import { invoke } from "@tauri-apps/api/core";
import { appConfigDir } from "@tauri-apps/api/path";


cachePlaylistTypes();

export async function playerPlayOrPause() {
    await invoke("player_play_or_pause");
}

export async function playerAdjustVolume(by: number) {
    await invoke("player_adjust_volume", { by });
}

export async function playerSeekPosition(by: number) {
    await invoke("player_seek_position", { by });
}

export async function playerSkip(to: number) {
    await invoke("player_skip", { to });
}

export async function playerSongPosition(): Promise<number> {
    return await invoke("player_song_position");
}

export async function playerCurrentSongInfo(key: string): Promise<any> {
    return await invoke("player_current_song_info", { key });
}

export async function playerShuffleQueue() {
    await invoke("player_shuffle_queue");
}

export async function playerCoverPathQueue(): Promise<any> {
    return await invoke("player_cover_path_queue");
}

export async function playerSongFinished(): Promise<boolean> {
    return await invoke("player_song_finished");
}

export async function playerRepeat(): Promise<boolean> {
    return await invoke("player_repeat");
}

export async function playerSongPaused(): Promise<boolean> {
    return await invoke("player_song_paused");
}

export async function playerToggleRepeat() {
    await invoke("player_toggle_repeat");
}

export async function getAlbumPlaylists(): Promise<any> {
    return await invoke("get_album_playlists");
}

export async function playAlbumPlaylist(album: string) {
    await invoke("play_album_playlist", { album });
}

export async function importCSS() {
    const configDir = await appConfigDir();
    let path = `"${configDir}/theme.css"`;
    let fallback = "/src/theme.css";

    document.head.insertAdjacentHTML(
        "beforeend",
        `'<link rel="stylesheet" href=${path} />'`,
    );

    if (!document.querySelector('link[href="${configDir}/theme.css"]')) {
        document.head.insertAdjacentHTML(
            "beforeend",
            `'<link rel="stylesheet" href=${fallback} />'`,
        );
    }
}

async function cachePlaylistTypes() {
    await invoke("create_playlist_types");
}
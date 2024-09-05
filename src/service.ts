import { invoke } from "@tauri-apps/api/core";

cachePlaylistTypes();

export async function playPause() {
    await invoke("play_pause");
}

export async function adjustVolume(by: number) {
    await invoke("adjust_volume", { by });
}

export async function seekPosition(nSeconds: number) {
    await invoke("seek_position", { nSeconds });
}

export async function getSongPosition(): Promise<number> {
    return await invoke("get_song_position");
}

export async function getCurrentSongInfo(key: string): Promise<any> {
    return await invoke("get_current_song_info", { key });
}

async function cachePlaylistTypes() {
    await invoke("create_playlist_types");
}

export async function skipMusic(toIndex: number) {
    await invoke("skip_music", { toIndex });
}

export async function shuffleMusic() {
    await invoke("shuffle_music");
}

export async function getQueue(): Promise<any> {
    return await invoke("get_queue_of_covers");
}

export async function playerNotPlaying(): Promise<boolean> {
    return await invoke("not_playing");
}

export async function getPlayerRepeat(): Promise<boolean> {
    return await invoke("get_repeat");
}

export async function togglePlayerRepeat() {
    await invoke("toggle_repeat");
}

export async function getAlbumPlaylists(): Promise<any> {
    return await invoke("get_album_playlists");
}

export async function playAlbumPlaylist(album: string) {
    await invoke("play_album_playlist", { album });
}
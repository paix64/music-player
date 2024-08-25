import { invoke } from "@tauri-apps/api/core";


export async function adjustVolume(by: number) {
    await invoke("adjust_volume", { by });
}

export async function seekMusic(pos: number) {
    pos = await getSongPosition() + pos;
    await invoke("seek_position", { pos });
}

async function getSongPosition(): Promise<any> {
    return await invoke("get_song_position");
}

export async function getCurrentSongInfo(key: string): Promise<any> {
    return await invoke("get_current_song_info", { key });
}
import { writable } from "svelte/store";

export const blocklist = writable<string[]>([])

export async function fetchBlocklist(): Promise<string[]> {
  const { invoke } = await import("@tauri-apps/api");
  return invoke('fetch_blocklist');
}

export async function addSiteToBlocklist(site: string) {
  const { invoke } = await import("@tauri-apps/api");
  return invoke('add_to_blocklist', { site });
}

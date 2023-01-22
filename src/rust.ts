import { invoke } from '@tauri-apps/api/tauri'
import { Config } from './data';

export class UserCache {
  readonly name: string;
  readonly uuid: string;
  readonly expiresOn: string;
  constructor(name: string, uuid: string, expiresOn: string) {
    this.name = name;
    this.uuid = uuid;
    this.expiresOn = expiresOn;
  }
}

export async function openDirDialog(): Promise<string> {
  return await invoke('open_dir_dialog');
}

export async function getUserCache(rootDir: string): Promise<UserCache[]> {
  return await invoke<UserCache[]>('read_usercache', { rootdir: rootDir })
}

export async function nameUUIDFromBytes(name: Uint8Array): Promise<string> {
  return await invoke<string>('name_uuid_from_bytes', { name: Array.from<number>(name) })
}

export async function nameUUIDFromString(name: string): Promise<string> {
  return await nameUUIDFromBytes(new TextEncoder().encode(name));
}

export async function fetch(url: string): Promise<string> {
  return await invoke<string>('fetch', { url });
}

export async function convert(config: Config): Promise<void> {
  return await invoke<void>('convert', { config });
}
import { fetch } from './rust'

export async function getMojangUUID(username: string): Promise<string> {
    const response = await fetch(`https://api.mojang.com/users/profiles/minecraft/${username}`);
    const json = await JSON.parse(response);
    return json.id.slice(0, 8) + '-' + json.id.slice(8, 12) + '-' + json.id.slice(12, 16) + '-' + json.id.slice(16, 20) + '-' + json.id.slice(20);
}
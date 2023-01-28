import { fetch, fetchPost } from './rust'

export async function getMojangUUID(username: string): Promise<string> {
    const response = await fetch(`https://api.mojang.com/users/profiles/minecraft/${username}`);
    const json = await JSON.parse(response);
    return json.id.slice(0, 8) + '-' + json.id.slice(8, 12) + '-' + json.id.slice(12, 16) + '-' + json.id.slice(16, 20) + '-' + json.id.slice(20);
}

export async function getYggdrasilProfileUUID(endpoint: string, username: string): Promise<string> {
    const response = await fetchPost(`${endpoint}/api/profiles/minecraft`, JSON.stringify([username]));
    const json = await JSON.parse(response)[0];
    return json.id.slice(0, 8) + '-' + json.id.slice(8, 12) + '-' + json.id.slice(12, 16) + '-' + json.id.slice(16, 20) + '-' + json.id.slice(20);
}
export type PlayerData = {
    name: string
    mojangUUID: string
    offlineUUID: string
}

export type Config = {
    rootDir: string,
    convertOptions: string[],
    uuids: Record<string, string>
}
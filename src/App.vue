<script setup lang="ts">
import { computed, reactive, Ref, ref, watch } from 'vue';
import ModeSelector from './components/ModeSelector.vue'
import Footer from './components/Footer.vue'
import { InputInst, NInput, NTable, NSpace, NButton, NAlert } from 'naive-ui';
import { UserCache, getUserCache, openDirDialog, nameUUIDFromString } from './rust';
import { getMojangUUID } from './mojang';
import { useAsyncState, UseAsyncStateReturn, computedAsync } from '@vueuse/core'

const mode = ref<'offline2online' | 'online2offline' | 'custom'>('offline2online');

const config = reactive<{
  rootDir: string
}>({
  rootDir: ''
})

const caches = reactive<UserCache[]>([])

const rootDirInputRef = ref<InputInst | null>(null)

async function onSelectRootDir() {
  rootDirInputRef.value?.blur()
  let res: string = await openDirDialog()
  config.rootDir = res

  let cache = await getUserCache(res)
  caches.splice(0, caches.length)
  caches.push(...cache)
}

type PlayerChecker = {
  name: string
  mojangUUID: UseAsyncStateReturn<string, true>
  offlineUUID: UseAsyncStateReturn<string, true>
}

const playerCheckers = reactive<PlayerChecker[]>([])

watch(caches, (newVal) => {
  playerCheckers.splice(0, playerCheckers.length)
  for (let cache of Array.from(new Set(newVal.map(it => it.name)))) {
    playerCheckers.push({
      name: cache,
      // @ts-ignore
      mojangUUID: useAsyncState<string>(getMojangUUID(cache), "加载中"),
      // @ts-ignore
      offlineUUID: useAsyncState<string>(nameUUIDFromString(`OfflinePlayer:${cache}`), "加载中")
    })
  }
})

const errors = computedAsync(async () => {
  let res: string[] = []
  if (config.rootDir === '') {
    res.push('服务端根目录不能为空')
  } else {
    try {
      await getUserCache(config.rootDir)
    } catch (err) {
      res.push('在指定服务端根目录中找不到 usercache.json 文件')
    }
  }
  return res
}, [])

</script>

<template>
  <div class="container">
    <ModeSelector v-model="mode" />
    <div class="content">
      <n-space vertical>
        <n-alert title="您需要修复以下错误，然后才能开始转换" type="error" v-if="errors.length !== 0">
          <template v-for="err in errors">
            <span style="display: block">- {{ err }}</span>
          </template>
        </n-alert>
        <div id="root-dir-selector" class="row space-between" style="align-items: center;">
          <span>服务端根目录</span>
          <n-input ref="rootDirInputRef" v-model:value="config.rootDir" type="text" placeholder="点击以选择服务端根目录文件夹"
            autosize style="min-width: 50%" @click="onSelectRootDir" />
        </div>
        <div class="players-checker" v-if="mode !== 'custom'">
          <n-table :bordered="false" :single-line="false">
            <thead>
              <tr>
                <th>玩家名</th>
                <th>当前 UUID</th>
                <th>将转换为 UUID</th>
                <th>玩家当前验证方式</th>
                <th>是否可以转换</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="info in playerCheckers">
                <td>{{ info.name }}</td>
                <template v-if="mode === 'offline2online'">
                  <td>{{ info.offlineUUID.state }}</td>
                  <td>
                    <template v-if="!info.mojangUUID.isReady && !info.mojangUUID.isLoading">
                      <span style="color: red">无法拉取正版账户信息</span>
                    </template>
                    <template v-else>
                      {{ info.mojangUUID.state }}
                    </template>
                  </td>
                </template>
                <template v-else-if="mode === 'online2offline'">
                  <td>
                    <template v-if="!info.mojangUUID.isReady && !info.mojangUUID.isLoading">
                      <span style="color: red">无法拉取正版账户信息</span>
                    </template>
                    <template v-else>
                      {{ info.mojangUUID.state }}
                    </template>
                  </td>
                  <td>{{ info.offlineUUID.state }}</td>
                </template>
                <td>{{ mode === 'offline2online' ? "离线" : "正版" }}</td>
                <template v-if="mode === 'offline2online'">
                  <td>{{ info.mojangUUID.isReady ? "✅" : "❌" }}</td>
                </template>
                <template v-else-if="mode === 'online2offline'">
                  <td>✅</td>
                </template>
              </tr>
            </tbody>
          </n-table>
        </div>
        <n-button type="primary" :disabled="errors.length !== 0">
          开始转换
        </n-button>
      </n-space>
    </div>
  </div>
  <Footer />
</template>

<style scoped>
.players-checker {
  max-height: 50vh;
  overflow-y: auto;
}
</style>

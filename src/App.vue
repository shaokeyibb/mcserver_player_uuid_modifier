<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import ModeSelector from './components/ModeSelector.vue'
import ErrorAlert from './components/ErrorAlert.vue';
import Footer from './components/Footer.vue'
import RootDirSelector from './components/RootDirSelector.vue';
import PlayerChecker from './components/PlayerChecker.vue';
import ConvertOptions from './components/ConvertOptions.vue';
import { NSpace, NButton, createDiscreteApi } from 'naive-ui';
import { convert, getUserCache } from './rust';
import { computedAsync } from '@vueuse/core'
import { Config } from './data';

const mode = ref<'offline2online' | 'online2offline' | 'custom'>('offline2online');

const convertOptions = [
  {
    label: "世界",
    value: 'world'
  }, {
    label: "插件（文本文件）",
    value: 'plugin_text'
  }, {
    label: "插件（SQLite）",
    value: 'plugin_sqlite',
    disabled: true
  }, {
    label: "插件（H2）",
    value: 'plugin_h2',
    disabled: true
  }, {
    label: "远程数据源",
    value: 'remote_datasource',
    disabled: true
  }]

const config = reactive<Config>({
  rootDir: '',
  convertOptions: [],
  uuids: {}
})

const rootDir = computed(() => config.rootDir)

const errors = computedAsync(async () => {
  let res: string[] = []

  if (Object.keys(config.uuids).length === 0) {
    res.push('请至少选择一个玩家数据进行转换')
  }

  if (config.convertOptions.length === 0) {
    res.push('请选择至少一个转换选项')
  }

  if (config.rootDir === '') {
    res.push('服务端根目录不能为空')
  } else {
    try { await getUserCache(config.rootDir) } catch (err) {
      res.push('在指定服务端根目录中找不到 usercache.json 文件')
    }
  }

  return res
}, [])

const input = reactive<string[]>([])

watch(rootDir, async (newVal) => {
  input.splice(0, input.length)
  let cache = await getUserCache(newVal)
  input.push(...Array.from(new Set(cache.map(it => it.name))))
}, {
  deep: true
})

const { loadingBar, notification } = createDiscreteApi(
  ['loadingBar', 'notification']
)

const running = ref(false)

async function handleStartConvert() {
  loadingBar.start()
  running.value = true
  try {
    await convert(config)
    loadingBar.finish()
  } catch (err) {
    loadingBar.error()
    notification['error'].create({
      title: "在转换玩家数据时发生了一个错误",
      content: err as string
    })
  }
  running.value = false
}

</script>

<template>
  <div class="container">
    <ModeSelector v-model="mode" />
    <div class="content">
      <n-space vertical>
        <ErrorAlert :error="errors" />
        <RootDirSelector v-model="config.rootDir" />
        <PlayerChecker :mode="mode" v-model:input="input" v-model:output="config.uuids" />
        <ConvertOptions v-model="config.convertOptions" :options="convertOptions" />
        <n-button type="primary" :disabled="errors.length !== 0 || running" @click="handleStartConvert">
          开始转换
        </n-button>
      </n-space>
    </div>
  </div>
  <Footer />
</template>

<style scoped>
.container {
  padding: 0 5vw;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  text-align: center;
  min-height: 85%;
}
</style>

<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { nameUUIDFromString } from '../rust';
import { getMojangUUID, getYggdrasilProfileUUID } from '../mojang';
import { computed, ref, watch } from 'vue';
import { NTable, NEmpty, createDiscreteApi } from 'naive-ui';

const props = defineProps<{
  mode: 'offline2online' | 'online2offline' | 'online2ygg' | 'ygg2online' | 'custom',
  input: string[],
  output: Record<string, string>,
  useExternal: boolean,
  externalYggdrasilLink: string
}>();

const emit = defineEmits<{
  (input: 'update:input', value: string[]): void,
  (output: 'update:output', value: Record<string, string>): void
}>();

type ConvertData = {
  name: string
  from: string | null
  to: string | null
}

const infoEvaluating = ref(false);
const _info = computedAsync<ConvertData[]>(async () => {
  let result: ConvertData[] = []
  for (let name of props.input) {

    let mojangUUID: string | null = null;

    if (props.useExternal) {
      try { mojangUUID = await getYggdrasilProfileUUID(props.externalYggdrasilLink, name) } catch { }
    } else {
      try { mojangUUID = await getMojangUUID(name) } catch { }
    }

    let offlineUUID = await nameUUIDFromString("OfflinePlayer:" + name);

    result.push({
      name: name,
      from: offlineUUID,
      to: mojangUUID
    });
  }

  return result;
}, [], infoEvaluating);
const info = computed<ConvertData[]>(() => {
  if (props.mode === 'offline2online') {
    return _info.value;
  }

  if (props.mode === 'online2offline') {
    return _info.value.map(it => ({
      name: it.name,
      from: it.to,
      to: it.from
    }));
  }

  // custom TODO
  return []
});

const yggInfoEvaluating = ref(false);
const _yggInfo = computedAsync<ConvertData[]>(async () => {
  let result: ConvertData[] = []
  for (let name of props.input) {

    let onlineUUID: string | null = null;
    let yggUUID: string | null = null;

    try { onlineUUID = await getMojangUUID(name) } catch { }
    try { yggUUID = await getYggdrasilProfileUUID(props.externalYggdrasilLink, name) } catch { }

    result.push({
      name: name,
      from: onlineUUID,
      to: yggUUID
    });
  }

  return result;
}, [], yggInfoEvaluating);
const yggInfo = computed<ConvertData[]>(() => {
  if (props.mode === 'online2ygg') {
    return _yggInfo.value;
  }

  if (props.mode === 'ygg2online') {
    return _yggInfo.value.map(it => ({
      name: it.name,
      from: it.to,
      to: it.from
    }));
  }

  // custom TODO
  return []
});

function onUpdateOutput(value: ConvertData[]) {
  emit('update:output', Object.fromEntries(value.filter(it => it.from != null && it.to != null).map(it => [it.from, it.to])));
}

const { loadingBar } = createDiscreteApi(
  ['loadingBar']
)

watch(infoEvaluating, () => {
  onEvaluating('info', infoEvaluating.value);
})
watch(yggInfoEvaluating, () => {
  onEvaluating('yggInfo', yggInfoEvaluating.value);
})

function onEvaluating(type: 'info' | 'yggInfo', newVal: boolean) {
  if (newVal) {
    loadingBar.start();
  } else {
    loadingBar.finish();
  }
}

const usedInfo = computed<ConvertData[]>(() => {
  if (props.mode === 'offline2online' || props.mode === 'online2offline') {
    return info.value;
  }

  if (props.mode === 'online2ygg' || props.mode === 'ygg2online') {
    return yggInfo.value;
  }

  // custom TODO
  return []
});

watch(usedInfo, (newVal) => {
  onUpdateOutput(newVal);
});

</script>

<template>
  <div id="players-checker">
    <template v-if="mode !== 'custom'">
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
          <tr v-for="i in usedInfo">
            <td>{{ i.name }}</td>

            <td>
              <template v-if="i.from == null">
                <span style="color: red">无法拉取远程账户信息</span>
              </template>
              <template v-else>
                {{ i.from }}
              </template>
            </td>

            <td>
              <template v-if="i.to == null">
                <span style="color: red">无法拉取远程账户信息</span>
              </template>
              <template v-else>
                {{ i.to }}
              </template>
            </td>

            <template v-if="mode === 'offline2online' || mode === 'online2offline'">
              <td>{{ mode === 'offline2online' ? "离线验证" : (useExternal ? "外置验证" : "正版验证") }}</td>
            </template>
            <template v-else>
              <td>{{ mode === 'online2ygg' ? "正版验证" : "外置验证" }}</td>
            </template>

            <td>{{ (i.from !== null && i.to !== null) ? "✅" : "❌" }}</td>
          </tr>
        </tbody>
      </n-table>
      <div id="no-data" v-if="input.length === 0">
        <n-empty description="无数据" />
      </div>
    </template>
  </div>
</template>

<style scoped>
#players-checker {
  max-height: 50vh;
  overflow-y: auto;
}

#no-data {
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #fff;
  padding: 10px;
}
</style>
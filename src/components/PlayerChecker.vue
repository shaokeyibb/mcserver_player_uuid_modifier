<script setup lang="ts">
import { computedAsync } from '@vueuse/core';
import { nameUUIDFromString } from '../rust';
import { getMojangUUID } from '../mojang';
import { computed, ref, watch } from 'vue';
import { NTable, useLoadingBar } from 'naive-ui';

const props = defineProps<{
  mode: 'offline2online' | 'online2offline' | 'custom',
  input: string[],
  output: Record<string, string>
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
    try { mojangUUID = await getMojangUUID(name) } catch { }

    let offlineUUID = await nameUUIDFromString(name);

    result.push({
      name: name,
      from: offlineUUID,
      to: mojangUUID
    });
  }

  return result;
}, [], infoEvaluating);
const info = computed(() => {
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

watch(info, (newVal) => {
  emit('update:output', Object.fromEntries(newVal.filter(it => it.from != null && it.to != null).map(it => [it.from, it.to])));
});

const loadingBar = useLoadingBar()
watch(infoEvaluating, (newVal) => {
  if (newVal) {
    loadingBar.start();
  } else {
    loadingBar.finish();
  }
})
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
          <tr v-for="i in info">
            <td>{{ i.name }}</td>

            <td>
              <template v-if="i.from == null">
                <span style="color: red">无法拉取正版账户信息</span>
              </template>
              <template v-else>
                {{ i.from }}
              </template>
            </td>

            <td>
              <template v-if="i.to == null">
                <span style="color: red">无法拉取正版账户信息</span>
              </template>
              <template v-else>
                {{ i.to }}
              </template>
            </td>

            <td>{{ mode === 'offline2online' ? "离线" : "正版" }}</td>

            <td>{{ (i.from !== null && i.to !== null) ? "✅" : "❌" }}</td>
          </tr>
        </tbody>
      </n-table>
      <div id="no-data" v-if="input.length === 0">
        <span>无数据</span>
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
  height: 30px;
  width: calc(100%);
}
</style>
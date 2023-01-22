<script setup lang="ts">
import { NAlert, NSpace } from 'naive-ui';

defineProps<{
    error: string[],
    warning: string[]
}>();
</script>

<template>
    <n-space vertical>
        <n-alert title="您需要修复以下错误，然后才能开始转换：" type="error" v-if="error.length !== 0">
            <template v-for="err in error">
                <span style="display: block">- {{ err }}</span>
            </template>
        </n-alert>
        <n-alert title="警告！开始转换前详阅：" type="warning" v-if="warning.length !== 0">
            <template v-for="warn in warning">
                <span style="display: block">- {{ warn }}</span>
            </template>
        </n-alert>
    </n-space>
</template>

<style scoped>

</style>
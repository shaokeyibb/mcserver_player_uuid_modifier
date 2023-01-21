<script setup lang="ts">
import { NAlert } from 'naive-ui';

defineProps<{
    error: string[]
}>();
</script>

<template>
    <n-alert title="您需要修复以下错误，然后才能开始转换" type="error" v-if="error.length !== 0">
        <template v-for="err in error">
            <span style="display: block">- {{ err }}</span>
        </template>
    </n-alert>
</template>

<style scoped>

</style>
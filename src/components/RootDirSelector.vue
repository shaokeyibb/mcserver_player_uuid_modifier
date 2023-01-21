<script setup lang="ts">
import { NInput, InputInst } from 'naive-ui';
import { ref } from 'vue';
import { openDirDialog } from '../rust';

defineProps<{
    modelValue: string
}>();

const emit = defineEmits<{
    (e: 'update:modelValue', value: string): void
}>();

const rootDirInputRef = ref<InputInst | null>(null)

async function onSelectRootDir() {
    rootDirInputRef.value?.blur()
    let res: string = await openDirDialog()
    emit('update:modelValue', res)
}
</script>

<template>
    <div id="root-dir-selector" class="row space-between" style="align-items: center;">
        <span>服务端根目录</span>
        <n-input ref="rootDirInputRef" :value="modelValue" @update:value="newVal => $emit('update:modelValue', newVal)"
            type="text" placeholder="点击以选择服务端根目录文件夹" style="max-width: 50%" @click="onSelectRootDir" />
    </div>
</template>
<script setup lang="ts">
import { NRadioGroup, NRadioButton } from 'naive-ui';

defineProps<{
    modelValue: 'offline2online' | 'online2offline' | 'custom'
}>();

defineEmits<{
    (e: 'update:modelValue', value: 'offline2online' | 'online2offline' | 'custom'): void
}>();
</script>

<template>
    <n-radio-group :value="modelValue" @update:value="newVal => $emit('update:modelValue', newVal)"
        style="margin-bottom: 12px" size="large">
        <n-radio-button value="offline2online">
            正版 UUID -> 盗版 UUID
        </n-radio-button>
        <n-radio-button value="online2offline">
            盗版 UUID -> 正版 UUID
        </n-radio-button>
        <n-radio-button value="custom" :disabled="true">
            自定义
        </n-radio-button>
    </n-radio-group>
</template>
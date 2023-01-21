<script setup lang="ts">
import { NSelect } from 'naive-ui';
import { SelectMixedOption } from 'naive-ui/es/select/src/interface';

defineProps<{
    modelValue: string[],
    options: SelectMixedOption[]
}>();

defineEmits<{
    (input: 'update:modelValue', value: string[]): void
}>();
</script>

<template>
    <div id="convert_options" class="row space-between" style="align-items: center;">
        <span>启用转换选项</span>
        <n-select :value="modelValue" @update:value="newVal => $emit('update:modelValue', newVal)" multiple
            :options="options" style="max-width: 50%" />
    </div>
</template>

<style scoped>

</style>
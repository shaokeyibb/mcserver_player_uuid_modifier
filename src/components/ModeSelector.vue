<script setup lang="ts">
import { NRadioGroup, NRadioButton, NSwitch, NInput, NSelect } from 'naive-ui';
import { computed } from 'vue';

const props = defineProps<{
    modelValue: 'offline2online' | 'online2offline' | 'online2ygg' | 'ygg2online' | 'custom',
    useExternal: boolean,
    externalYggdrasilLink: string
}>();

defineEmits<{
    (e: 'update:modelValue', value: 'offline2online' | 'online2offline' | 'online2ygg' | 'ygg2online' | 'custom'): void,
    (e: 'update:useExternal', value: boolean): void,
    (e: 'update:externalYggdrasilLink', value: string): void
}>();

const options = [
    {
        label: "离线 UUID -> 外置验证 UUID",
        value: "offline2online"
    },
    {
        label: "外置验证 UUID -> 离线 UUID",
        value: "online2offline"
    },
    {
        label: "正版验证 UUID -> 外置验证 UUID",
        value: "online2ygg"
    },
    {
        label: "外置验证 UUID -> 正版验证 UUID",
        value: "ygg2online"
    },
    {
        label: "自定义",
        value: "custom",
        disabled: true
    }
]

const onlineText = computed(() => {
    if (props.useExternal) {
        return '外置验证';
    } else {
        return '正版验证';
    }
});
</script>

<template>
    <div id="selector">
        <div id="chooser">
            <template v-if="useExternal">
                <n-select :value="modelValue" @update:value="newVal => $emit('update:modelValue', newVal)"
                    :options="options" />
            </template>
            <template v-else>
                <n-radio-group :value="modelValue" @update:value="newVal => $emit('update:modelValue', newVal)"
                    style="margin-bottom: 12px" size="large">
                    <n-radio-button value="offline2online">
                        离线 UUID -> {{ onlineText }} UUID
                    </n-radio-button>
                    <n-radio-button value="online2offline">
                        {{ onlineText }} UUID -> 离线 UUID
                    </n-radio-button>
                    <n-radio-button value="custom" :disabled="true">
                        自定义
                    </n-radio-button>
                </n-radio-group>
            </template>
        </div>
        <n-switch :value="useExternal" @update:value="newVal => $emit('update:useExternal', newVal)"
            style="margin-bottom: 22px; align-self: flex-end;">
            <template #checked>
                外置验证
            </template>
            <template #unchecked>
                正版验证
            </template>
        </n-switch>
    </div>
    <div id="yggdrasil-selector" class="row space-between" style="align-items: center;" v-if="useExternal">
        <span>自定义 Yggdrasil 服务地址</span>
        <n-input :value="externalYggdrasilLink" @update:value="newVal => $emit('update:externalYggdrasilLink', newVal)"
            type="text" placeholder="https://littleskin.cn/api/yggdrasil" style="max-width: 50%" />
    </div>
</template>

<style scoped>
#selector {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    height: 52px;
}

#chooser {
    width: 518.21px
}
</style>
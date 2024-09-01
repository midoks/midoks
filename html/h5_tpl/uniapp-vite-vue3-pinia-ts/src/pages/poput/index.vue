<template>
    <div>
        <wd-navbar
            safe-area-inset-top
            title="标题"
            left-text="返回"
            left-arrow
        ></wd-navbar>

        <wd-form ref="form" :model="model">
            <wd-cell-group border>
                <wd-input
                    v-model="model.value1"
                    label="校验"
                    label-width="100px"
                    prop="value1"
                    clearable
                    placeholder="正则校验"
                    :rules="[
                        {
                            required: false,
                            pattern: /\d{6}/,
                            message: '请输入6位字符',
                        },
                    ]"
                />
                <wd-input
                    v-model="model.value2"
                    label="校验"
                    label-width="100px"
                    prop="value2"
                    clearable
                    placeholder="函数校验"
                    :rules="[
                        {
                            required: false,
                            validator: validatorMessage,
                            message: '请输入正确的手机号',
                        },
                    ]"
                />
                <wd-input
                    v-model="model.value3"
                    label="校验"
                    label-width="100px"
                    prop="value3"
                    clearable
                    placeholder="校验函数返回错误提示"
                    :rules="[
                        {
                            required: false,
                            message: '请输入内容',
                            validator: validator,
                        },
                    ]"
                />
                <wd-input
                    v-model="model.value4"
                    label="校验"
                    label-width="100px"
                    prop="value4"
                    clearable
                    placeholder="异步函数校验"
                    :rules="[
                        {
                            required: false,
                            validator: asyncValidator,
                            message: '请输入1234',
                        },
                    ]"
                />
            </wd-cell-group>
            <view class="footer">
                <wd-button
                    type="primary"
                    size="large"
                    block
                    @click="handleSubmit"
                    >提交</wd-button
                >
            </view>
        </wd-form>

        <wd-toast />
    </div>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue';
import { useToast } from 'wot-design-uni';

const model = reactive<{
    value1: string;
    value2: string;
    value3: string;
    value4: string;
}>({
    value1: '',
    value2: '',
    value3: '',
    value4: '',
});

const {
    success: showSuccess,
    loading: showLoading,
    close: closeToast,
} = useToast();

const form = ref();

const validatorMessage = (val: any) => {
    return /1\d{10}/.test(val);
};

const validator = (val: any) => {
    if (String(val).length >= 4) {
        return Promise.resolve();
    } else {
        return Promise.reject('长度不得小于4');
    }
};

// 校验函数可以返回 Promise，实现异步校验
const asyncValidator = (val: any) =>
    new Promise((resolve) => {
        showLoading('验证中...');
        setTimeout(() => {
            closeToast();
            resolve(val === '1234');
        }, 1000);
    });

function handleSubmit() {
    form.value
        .validate()
        .then(({ valid }: any) => {
            if (valid) {
                showSuccess({
                    msg: '提交成功',
                });
            }
        })
        .catch((error: any) => {
            console.log(error, 'error');
        });
}
</script>

<style lang="scss" scoped>
.footer {
    padding: 12px;
}
</style>

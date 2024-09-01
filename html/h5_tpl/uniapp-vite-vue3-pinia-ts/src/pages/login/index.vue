<!--
 * @Description: 
 * @Version: V1.0.0
 * @Author: 周艳凯 750419898@qq.com
 * @Date: 2023-07-26 17:37:47
 * @LastEditors: 周艳凯 750419898@qq.com
 * @LastEditTime: 2024-03-13 10:24:49
 * @FilePath: index.vue
 * Copyright 2024 Marvin, All Rights Reserved. 
 * 2023-07-26 17:37:47
-->
<template>
    <view class="contain">
        <view class="con">
            <view class="content rounded-lg">
                <view class="title text-3xl text-teal-500 my-4"> Sign In </view>

                <view class="ip-info">
                    <view class="inps">
                        <input v-model="form.name" placeholder="Username" />
                    </view>
                    <view class="inps">
                        <input v-model="form.passWord" placeholder="Password" />
                    </view>
                </view>

                <view class="btns">
                    <button
                        class="bg-violet-300 w-auto rounded-lg text-zinc-50"
                        @click="login"
                    >
                        登录
                    </button>
                </view>

                <view class="desc">
                    <text>
                        Don't have an account?
                        <text class="text-violet-500"> Sign Up </text>
                    </text>
                </view>
            </view>
        </view>
    </view>
</template>

<script lang="ts" setup name="login">
import { reactive } from 'vue';
import { useAuthStore } from '@/state/modules/user';
import { useRouter } from 'uni-mini-router';
const router = useRouter();
const authStore = useAuthStore();
const form = reactive({
    name: '小团体',
    passWord: '123456',
});

// var name = 12321;

// login登录
function login() {
    authStore
        .login(form)
        .then((res) => {
            console.log(res, 'wi使用了');
            router.push({
                name: 'home',
            });
        })
        .catch(() => {});
}
</script>

<style lang="scss" scoped>
.contain {
    width: 100%;
    height: 100vh;
    background: url('@/static/back.jpg') no-repeat;
    background-size: cover;

    .con {
        width: 100%;
        height: 100%;
        background: rgb(0 0 0 / 40%);

        .content {
            position: absolute;
            top: 200px;
            right: 30rpx;
            left: 30rpx;
            padding: 30rpx;
            background: #fff;

            .ip-info {
                margin-bottom: 50rpx;

                .inps {
                    padding: 20rpx;
                    border-bottom: 2rpx solid rgb(204 204 204 / 100%);
                }
            }

            .btns {
                margin-bottom: 30rpx;
            }

            .desc {
                text-align: center;
            }
        }
    }
}
</style>

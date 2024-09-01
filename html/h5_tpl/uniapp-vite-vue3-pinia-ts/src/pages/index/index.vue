<template>
    <view class="content">
        <image
            class="logo rounded-lg"
            :src="useAuth.user.logo"
            @click="showPreview = true"
        />
        <view class="text-area">
            <text class="title font-mono text-purple-500 text-4xl">
                {{ title }}
            </text>
        </view>

        <!-- <view>
      <text class="font-mono text-purple-500 ">
           路由  uni-mini-router
      </text>
    
      <text>文档地址： https://fant-mini-plus.top/fant-mini-plus/uni-mini-router/introduction.html</text>

    </view> -->

        <wd-icon name="search" size="20px"></wd-icon>

        <view class="text-area">
            <text class="title font-mono text-rose-500 text-2xl">
                用户名 {{ useAuth.user.name }}
            </text>
        </view>
        <view class="text-area">
            <text class="title font-mono text-rose-500 text-2xl">
                用户密码 {{ useAuth.user.passWord }}
            </text>
        </view>
        <view class="text-area">
            <text class="title font-mono text-rose-500 text-2xl">
                用户手机号 {{ useAuth.user.phone }}
            </text>
        </view>

        <view class="text-area">
            <text class="title font-mono text-rose-500 text-2xl">
                {{ usetsto.getNum }}
            </text>
        </view>

        <view class="my-1">
            <wd-button class="1231" color="#7232dd" @click="usetsto.addnum">
                pinia++
            </wd-button>
        </view>
        <view class="my-1">
            <!-- <tn-button color="#7232dd" > 跳转路由 </tn-button> -->
            <wd-button type="success" @click="push">跳转路由</wd-button>
        </view>
        <view class="my-1">
            <!-- <tn-button color="#7232dd" "> 弹出层 </tn-button> -->
            <wd-button type="success" @click="pushs">弹出层</wd-button>
        </view>
        <view class="my-1">
            <!-- <tn-button color="#7232dd" @click="apis">  </tn-button> -->
            <wd-button type="success" @click="apis">api请求</wd-button>
        </view>

        <!-- <vin-image-preview
            :show="showPreview"
            :images="imgData"
            @close="showPreview = false"
        /> -->

        <view class="text-area my-1">
            <text class="title font-mono text-rose-500 text-1xl">
                {{ env }}
            </text>
        </view>

        <view class="text-area my-1">
            <text class="title font-mono text-rose-500 text-1xl">
                {{ api }}
            </text>
        </view>

        <wd-button label="按钮"></wd-button>

        <!-- <tn-button type="primary">按钮</tn-button> -->
        <!-- <TnTitle title="图鸟UI" mode="vLine" /> -->
    </view>
</template>

<script setup lang="ts" name="index">
import { ref } from 'vue';
import { useRouter } from 'uni-mini-router';
// import coas from '@/static/logo.png';
import { useAppStore } from '@/state/app';
import { useAuthStore } from '@/state/modules/user';
import { getlist } from '@/api/common/user/user';

const router = useRouter();
const usetsto = useAppStore();
const useAuth = useAuthStore();

let env = import.meta.env.VITE_APP_ENV;
let api = import.meta.env.VITE_BASE_URL;

const title = ref('Tailwind CSS');

const showPreview = ref(false);

// let name = 1312;

// let name = 1321;

// const imgData = [
//     {
//         src: coas,
//     },
// ];

// 跳转路由
function push() {
    console.log('我点击跳转路由了？');
    router.push({
        name: 'wode',
    });
}

// 跳转路由
function pushs() {
    console.log('我点击跳转路由了？');
    router.push({
        name: 'poput',
    });
}

// 请求api
function apis() {
    getlist({ id: 1 }).then((res) => {
        console.log(res, '我点击了');
    });
}
</script>

<style>
.content {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
}

.logo {
    margin: 200rpx auto 50rpx;
    width: 200rpx;
    height: 200rpx;
}

.text-area {
    display: flex;
    justify-content: center;
}

:deep(.vin-image-preview-custom-pop) {
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    background: transparent !important;
}
</style>

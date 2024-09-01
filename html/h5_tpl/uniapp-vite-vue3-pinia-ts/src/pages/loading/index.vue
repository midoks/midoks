<template>
    <div class="load-contain">
        <div class="loading-wave">
            <div class="loading-bar"></div>
            <div class="loading-bar"></div>
            <div class="loading-bar"></div>
            <div class="loading-bar"></div>
        </div>
    </div>
</template>

<script lang="ts" setup>
console.log('我进来loading也没了');
import { useRouter } from 'uni-mini-router';
import { useAuthStore } from '@/state/modules/user';
import { onMounted } from 'vue';
const router = useRouter();
const authStore = useAuthStore();

onMounted(() => {
    console.log(authStore.isLogin, '我来跳转了？？');
    setTimeout(() => {
        if (authStore.isLogin) {
            // 如果登录了就放行
            router.replace({
                name: 'home',
            });
        } else {
            router.replace({
                name: 'login',
            });
        }
    }, 800);
});

// console.log('我要进来么啊，1');
// 判断是否需要登录
</script>

<style lang="scss" scoped>
.load-contain {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
}

.loading-wave {
    display: flex;
    justify-content: center;
    align-items: flex-end;
    width: 300px;
    height: 60px;
}

.loading-bar {
    margin: 0 5px;
    width: 20px;
    height: 10px;
    border-radius: 5px;
    background-color: #3498db;
    animation: loading-wave-animation 1s ease-in-out infinite;
}

.loading-bar:nth-child(2) {
    animation-delay: 0.1s;
}

.loading-bar:nth-child(3) {
    animation-delay: 0.2s;
}

.loading-bar:nth-child(4) {
    animation-delay: 0.3s;
}

@keyframes loading-wave-animation {
    0% {
        height: 10px;
    }

    50% {
        height: 50px;
    }

    100% {
        height: 10px;
    }
}
</style>

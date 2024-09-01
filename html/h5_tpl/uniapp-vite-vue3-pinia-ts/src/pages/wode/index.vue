<template>
    <view class="contain">
        <view class="p-2">
            <view class="top">
                <image :src="useAuth.user.logo" />
                <view>{{ useAuth.user.name }}</view>
                <view>{{ useAuth.user.phone }}</view>
            </view>
            <view class="center">
                <view class="cen-title">Join Premium plan with us</view>
                <view class="flex cen-botton">
                    <view class="price"> $ 30.00</view>
                    <view class="cen-btns">Get Started</view>
                </view>
            </view>
            <view class="list">
                <wd-cell-group class="list" inset>
                    <wd-cell title="我的" clickable :is-link="true" />
                    <wd-cell title="扫一扫" clickable :is-link="true" />
                    <wd-cell title="修改密码" clickable :is-link="true" />
                </wd-cell-group>
            </view>

            <view class="bottom">
                <wd-button
                    round
                    block
                    type="primary"
                    plain
                    @click="handleLoginOut"
                >
                    {{ isLogin ? '退出登录' : '登录' }}
                </wd-button>
            </view>
        </view>
    </view>
</template>

<script lang="ts" setup>
import { useAuthStore } from '@/state/modules/user';
import { storeToRefs } from 'pinia';
import { useRouter } from 'uni-mini-router';
const useAuth = useAuthStore();
const router = useRouter();
const { isLogin } = storeToRefs(useAuth);
const { layout } = useAuth;
// 退出登录
function handleLoginOut() {
    if (!isLogin.value) {
        return router.replaceAll({
            name: 'login',
        });
    }
    layout();
}
</script>

<style lang="scss" scoped>
.contain {
    height: 100%;
    background: #fafafa;
}

.top {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin-top: 200rpx;
    margin-bottom: 40rpx;

    image {
        margin-bottom: 25rpx;
        width: 120rpx;
        height: 120rpx;
        border-radius: 50%;
    }
}

.center {
    display: flex;
    justify-content: space-between;
    padding: 20rpx;
    height: 224rpx;
    border-radius: 10rpx;
    color: #fff;
    background: #54bcbd;
    flex-direction: column;

    .cen-botton {
        justify-content: space-between;
        margin-top: auto;

        .price {
            font-size: 38rpx;
        }

        .cen-btns {
            padding: 15rpx 20rpx;
            border-radius: 20rpx;
            color: #54bcbd;
            background: #fff;
        }
    }
}

.bottom {
    margin-top: 100rpx;
    width: 100%;
}
</style>

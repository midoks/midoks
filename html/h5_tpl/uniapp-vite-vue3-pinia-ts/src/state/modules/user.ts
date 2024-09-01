/*
 * @Author: zhouyankai
 * @Date: 2023-07-26
 * @LastEditTime: 2024-03-13 10:32:00
 * @LastEditors: 周艳凯 750419898@qq.com
 * @Description:有关权限的pinia
 */
import { defineStore } from 'pinia';

interface AuthState {
    token?: string;
    user: Record<string, any>;
}

export const useAuthStore = defineStore({
    id: 'auth',
    state: (): AuthState => ({
        token: undefined,
        user: {},
    }),
    getters: {
        isLogin(): boolean {
            return this.token !== undefined;
        },
    },
    actions: {
        // 登录
        login(info: Records) {
            return new Promise<void>((resolve) => {
                setTimeout(() => {
                    this.token = '132131';
                    this.user = {
                        name: info.name,
                        passWord: info.passWord,
                        phone: 17703787046,
                        logo: 'https://img0.baidu.com/it/u=4079113664,1534334121&fm=253&fmt=auto&app=120&f=JPEG?w=400&h=400',
                    };
                    resolve();
                }, 2000);
            });
        },
        layout() {
            this.token = undefined;
            this.user = {};
        },
    },
    // 本地持久化存储
    persist: {
        storage: {
            getItem: uni.getStorageSync,
            setItem: uni.setStorageSync,
        },
    },
});

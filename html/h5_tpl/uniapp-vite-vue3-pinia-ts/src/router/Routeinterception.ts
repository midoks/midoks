// 路由拦截
import type { Router } from 'uni-mini-router';
import { useAuthStore } from '@/state/modules/user';
// 设置一些白名单
const whiteList = ['login', 'wode', 'loading'];

export function userRouternext(router: Router) {
    router.beforeEach((to, from, next) => {
        // console.log('我是否进来路由前置收尾了');
        // next入参 false 以取消导航
        const authStore = useAuthStore();
        // console.log('我要进来么啊，1');
        // 判断是否需要登录
        if (authStore.isLogin) {
            // 如果登录了就放行
            next();
        } else {
            // 判断一下是否白名单里面的东西 如果是的话就代表直接跳转无需判断是否有token
            if (whiteList.includes(to.name as string)) {
                // 在免登录白名单，直接进入
                // console.log('我在白名单里面');
                next();
            } else {
                // 判断不是白名单 还没有token值 直接进去登录页面页面
                next({
                    name: 'loading',
                });
            }
        }

        // next();
    });

    router.afterEach((to) => {
        // console.log(from);

        const authStore = useAuthStore();

        console.log('我先执行还是他先执行');
        // console.log(authStore.isLogin, to.name, '我跳转了');

        if (!authStore.isLogin && to && to.name !== 'login') {
            // 如果没有登录且目标路由不是登录页面则跳转到登录页面

            if (whiteList.includes(to.name as string)) {
                // 在免登录白名单，直接进入
                // console.log('我在白名单里面');
            } else {
                router.push({
                    name: 'loading',
                    params: { ...to.query },
                });
            }
        } else if (authStore.isLogin && to && to.name === 'login') {
            // 如果已经登录且目标页面是登录页面则跳转至首页
            router.replaceAll({ name: 'home' });
        }
    });
}

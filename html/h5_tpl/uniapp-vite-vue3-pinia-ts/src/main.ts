import { createSSRApp } from 'vue';
import App from './App.vue';
import pinia from './state/index'; //pinia
import router from './router'; //路由
export function createApp() {
    const app = createSSRApp(App);
    app.use(pinia);
    app.use(router);

    return {
        app,
    };
}

import Request from 'luch-request'; //npm下载引入luch-request

// const api = import.meta.env.VITE_BASE_URL;

// import qs from 'qs'
const http = new Request({
    baseURL: import.meta.env.VITE_BASE_URL, //设置请求的base url
    timeout: 30000, //超时时长30秒,
    header: {
        'Content-Type': 'multipart/form-data;application/json;charset=UTF-8;',
    },
});

//请求拦截器
http.interceptors.request.use(
    (config) => {
        // 可使用async await 做异步操作
        const token = uni.getStorageSync('token');
        if (token) {
            // config.header.common['Authorization'] = 'Bearer ' + token;
        }

        if (config.method === 'POST') {
            config.data = JSON.stringify(config.data) as unknown as Record<
                string,
                any
            >;
        }
        return config;
    },
    (error) => {
        return Promise.resolve(error);
    },
);

// 响应拦截器
http.interceptors.response.use(
    (response) => {
        console.log(response);
        return response;
    },
    (error) => {
        //未登录时清空缓存跳转
        if (error.statusCode == 401) {
            uni.clearStorageSync();
            uni.switchTab({
                url: '/pages/index/index.vue',
            });
        }

        const { errMsg } = error;

        console.log(errMsg, '我才是error');

        uni.showToast({
            title: errMsg,
            duration: 2000,
        });
        return Promise.resolve(error);
    },
);
export default http;

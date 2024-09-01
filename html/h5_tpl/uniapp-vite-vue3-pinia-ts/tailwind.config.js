/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./public/index.html', './src/**/*.{html,js,ts,jsx,tsx,vue}'],
    theme: {
        extend: {},
    },
    plugins: [],
    corePlugins: {
        // 不需要 preflight，因为这主要是给 h5 的，如果你要同时开发小程序和 h5 端，你应该使用环境变量来控制它
        preflight: false,
    },
};

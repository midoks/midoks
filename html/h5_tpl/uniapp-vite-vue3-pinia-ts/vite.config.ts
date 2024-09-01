import { defineConfig } from 'vite';
import { resolve } from 'path';
import uni from '@dcloudio/vite-plugin-uni';
import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';
import rem2px from 'postcss-rem-to-responsive-pixel';
import { UnifiedViteWeappTailwindcssPlugin as uvwt } from 'weapp-tailwindcss/vite';
import TransformPages from 'uni-read-pages-vite';
import eslintPlugin from 'vite-plugin-eslint'; //eslint自动检查插件

const isH5 = process.env.UNI_PLATFORM === 'h5';
const isApp = process.env.UNI_PLATFORM === 'app';
const WeappTailwindcssDisabled = isH5 || isApp;

const postcssPlugins = [tailwindcss(), autoprefixer()];
if (!WeappTailwindcssDisabled) {
    postcssPlugins.push(
        rem2px({
            // 32 意味着 1rem = 32rpx
            rootValue: 32,
            // 默认所有属性都转化
            propList: ['*'],
            // 转化的单位,可以变成 px / rpx
            transformUnit: 'rpx',
        }),
    );
}

// https://vitejs.dev/config/
export default defineConfig({
    server: {
        //跨域配置
        host: '0.0.0.0',
        port: 5174,
        open: true, //是否自动打开浏览器
    },
    build: {
        minify: 'terser',
        terserOptions: {
            compress: {
                drop_console: true,
            },
        },
        rollupOptions: {
            output: {
                chunkFileNames: 'assets/js/[name]-[hash].js',
                entryFileNames: 'assets/js/[name]-[hash].js',
                assetFileNames: 'assets/[ext]/[name]-[hash].[ext]',
            },
        },
        //   关闭文件计算
        reportCompressedSize: false,
        //   关闭生成map文件 可以达到缩小打包体积
        sourcemap: false, // 这个生产环境一定要关闭，不然打包的产物会很大
    },
    // 路径配置
    resolve: {
        alias: [
            {
                find: '@',
                replacement: resolve(__dirname, 'src'), //配置@ 路径
            },
        ],
    },
    define: {
        ROUTES: new TransformPages().routes, // 注入路由表
    },
    plugins: [
        uni(),
        uvwt({
            disabled: WeappTailwindcssDisabled,
        }),
        eslintPlugin({
            include: ['src/**/*.ts', 'src/**/*.vue', 'src/*.ts', 'src/*.vue'],
        }),
    ],
    css: {
        postcss: {
            plugins: postcssPlugins,
        },
    },
});

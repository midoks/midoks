import { defineConfig } from "vite";
import { resolve } from 'path';
import uni from "@dcloudio/vite-plugin-uni";
import AutoImport from "unplugin-auto-import/vite";
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    uni(),
    AutoImport({
      imports: ["vue", "vue-router"],
    }),
  ],
  resolve: {
    alias: [
      {
        find: "@",
        replacement: resolve(__dirname, "src"), //配置@ 路径
      },
    ],
  },
});

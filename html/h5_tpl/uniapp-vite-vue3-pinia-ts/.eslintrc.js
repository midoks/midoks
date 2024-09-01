module.exports = {
    env: {
        browser: true,
        es2021: true,
        node: true,
    },
    /**定义ESLint的解析器 */
    parser: 'vue-eslint-parser',
    extends: [
        'eslint:recommended',
        'plugin:vue/vue3-recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:prettier/recommended', // 新增
    ],
    parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        parser: '@typescript-eslint/parser',
    },
    plugins: ['vue', '@typescript-eslint'],
    rules: {
        'no-var': 'warn', // 禁止出现var用let和const代替
        'no-unused-vars': 'off', //关闭自带的规则
        '@typescript-eslint/no-unused-vars': 'error', //开启未使用变量的规则
        '@typescript-eslint/no-empty-interface': 'off',
        'vue/multi-word-component-names': 'off', //根节点的name
        '@typescript-eslint/no-empty-function': 'off', //不允许空函数
        '@typescript-eslint/no-explicit-any': 'off', //关闭使用any报错的规则
    },
};

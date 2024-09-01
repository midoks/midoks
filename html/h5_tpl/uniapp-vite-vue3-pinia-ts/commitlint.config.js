// type 类别：用于表明本次提交做了那种类型的改动。
// - feat：新增功能
// - fix：缺陷修复
// - perf：性能优化
// - refactor：重构代码(既没有新增功能，也没有修复 bug)
// - style：不影响程序逻辑的代码修改(代码风格样式等，没有改变代码逻辑)
// - docs：文档更新
// - build：项目构建系统(例如 glup，webpack，rollup 的配置等)的提交
// - revert：回滚某个更早之前的提交
// - chore：不属于以上类型的其他类型

// optional scope：一个可选的修改范围。用于标识此次提交主要涉及到代码中哪个模块。

// description：一句话描述此次提交的主要内容，做到言简意赅。

module.exports = {
    ignores: [(commit) => commit.includes('init')],
    extends: ['@commitlint/config-conventional'],
    rules: {
        'body-leading-blank': [2, 'always'],
        'footer-leading-blank': [1, 'always'],
        'header-max-length': [2, 'always', 108],
        'subject-empty': [2, 'never'],
        'type-empty': [2, 'never'],
        'type-enum': [
            2,
            'always',
            [
                'feat',
                'fix',
                'perf',
                'style',
                'docs',
                'test',
                'refactor',
                'build',
                'ci',
                'chore',
                'revert',
                'wip',
                'workflow',
                'types',
                'release',
            ],
        ],
    },
};

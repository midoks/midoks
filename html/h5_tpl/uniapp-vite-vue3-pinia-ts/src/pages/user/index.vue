<!-- 自定义下拉刷新与上拉加载演示(vue) -->
<template>
    <view class="content">
        <loadings v-if="loading"></loadings>
        <z-paging
            v-else
            ref="paging"
            v-model="dataList"
            :fixed="false"
            @query="queryList"
        >
            <!-- 需要固定在顶部不滚动的view放在slot="top"的view中，如果需要跟着滚动，则不要设置slot="top" -->
            <!-- 注意！此处的z-tabs为独立的组件，可替换为第三方的tabs，若需要使用z-tabs，请在插件市场搜索z-tabs并引入，否则会报插件找不到的错误 -->
            <template #top>
                <wd-navbar
                    safe-area-inset-top
                    title="我的"
                    left-arrow
                ></wd-navbar>
                <wd-tabs v-model="tabIndex" @change="tabsChange">
                    <block v-for="item in tabList" :key="item">
                        <wd-tab :title="`标签${item}`">
                            <!-- <view class="content">内容{{ item }}</view> -->
                        </wd-tab>
                    </block>
                </wd-tabs>
            </template>
            <!-- 自定义下拉刷新view(如果use-custom-refresher为true且不设置下面的slot="refresher"，此时不用获取refresherStatus，会自动使用z-paging自带的下拉刷新view) -->
            <!-- 注意注意注意！！字节跳动小程序中自定义下拉刷新不支持slot-scope，将导致custom-refresher无法显示 -->
            <!-- 如果是字节跳动小程序，请参照sticky-demo.vue中的写法，此处使用slot-scope是为了减少data中无关变量声明，降低依赖 -->
            <template #refresher="{ refresherStatus }">
                <!-- 此处的custom-refresh为demo中自定义的组件，非z-paging的内置组件，请在实际项目中自行创建。这里插入什么view，下拉刷新就显示什么view -->
                <custom :status="refresherStatus" />
            </template>
            <!-- 自定义没有更多数据view -->
            <template #loadingMoreNoMore>
                <!-- 此处的custom-nomore为demo中自定义的组件，非z-paging的内置组件，请在实际项目中自行创建。这里插入什么view，没有更多数据就显示什么view -->
                <mode />
            </template>

            <!-- 如果希望其他view跟着页面滚动，可以放在z-paging标签内 -->
            <view v-for="(item, index) in dataList" :key="index" class="item">
                <view class="item-title text-base">{{ item.title }}</view>
                <view class="item-detail">{{ item.detail }}</view>
                <view class="item-line"></view>
            </view>
        </z-paging>
    </view>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import request from './ces';
import custom from '@/components/custom-refresher/index.vue';
import mode from '@/components/custom-nomore/index.vue';
import loadings from '@/components/loading/index.vue';
import loadingss from '@/hooks/loading';
const { loading, setLoading } = loadingss();
const paging = ref<any>(null);
const tabIndex = ref(0);
const tabList = ref(['测试1', '测试2', '测试3', '测试4']);
// v-model绑定的这个变量不要在分页请求结束中自己赋值！！！
const dataList = ref<any[]>([]);

console.log('\x1B[41m\x1B[37m白色文字，红色背景\x1B[0m');

const tabsChange = (index) => {
    tabIndex.value = index;
    // 当切换tab或搜索时请调用组件的reload方法，请勿直接调用：queryList方法！！
    paging.value?.reload(true);
};

setLoading(true);

onMounted(() => {
    setTimeout(() => {
        setLoading(false);
        console.log('我非得要住宿');
    }, 3000);
});

// @query所绑定的方法不要自己调用！！需要刷新列表数据时，只需要调用paging.value.reload()即可
const queryList = (pageNo, pageSize) => {
    // 组件加载时会自动触发此方法，因此默认页面加载时会自动触发，无需手动调用
    // 这里的pageNo和pageSize会自动计算好，直接传给服务器即可
    // 模拟请求服务器获取分页数据，请替换成自己的网络请求
    const params = {
        pageNo: pageNo,
        pageSize: pageSize,
        type: tabIndex.value + 1,
    };
    request
        .queryList(params)
        .then((res: any) => {
            // 将请求的结果数组传递给z-paging
            paging.value.complete(res.data.list);
        })
        .catch(() => {
            // 如果请求失败写paging.value.complete(false);
            // 注意，每次都需要在catch中写这句话很麻烦，z-paging提供了方案可以全局统一处理
            // 在底层的网络请求抛出异常时，写uni.$emit('z-paging-error-emit');即可
            paging.value.complete(false);
        });
};
</script>

<style>
.content {
    height: 100%;
}

.item {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0rpx 30rpx;
    height: 150rpx;
}

.item-detail {
    padding: 5rpx 15rpx;
    font-size: 28rpx;
    border-radius: 10rpx;
    color: white;
    background-color: #007aff;
}

.item-line {
    position: absolute;
    bottom: 0rpx;
    left: 0rpx;
    width: 100%;
    height: 1px;
    background-color: #eee;
}
</style>

import request from '@/utils/request';

const BASE_API = '/mini/api/mall/order';

export default {
  tab(params) {
    return request({
      url: BASE_API + '/getTabCondition',
      method: 'get',
      params: params,
    });
  },
  page(params, headers) {
    return request({
      url: BASE_API + '/page',
      method: 'get',
      params: params,
      headers: headers,
    });
  },
  detail(orderNo) {
    return request({
      url: BASE_API + '/' + orderNo,
      method: 'get',
    });
  },
  create(data) {
    return request({
      url: BASE_API + '/create',
      method: 'post',
      data: data,
    });
  },
  cancel(data) {
    return request({
      url: BASE_API + '/cancel',
      method: 'put',
      data,
    });
  },
  pay(data) {
    return request({
      url: BASE_API + '/pay',
      method: 'post',
      data: data,
    });
  },
  // TODO 仅测试环境使用！！！
  payTest(data) {
    return request({
      url: BASE_API + '/payTest',
      method: 'post',
      data: data,
    });
  },
};

// import request from '@/http/httpload.js'
import request from '@/utils/request'; //导入http下的httpload.js

export function getlist(data: Record<string, any>) {
    // get请求 可以拼接url或者传入数据对象 二选一
    return request.get('/list', { params: data });
}

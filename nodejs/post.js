var querystring = require('querystring');
 var http = require('http');

var lhc_query = querystring.stringify({
    'year': '2018',
    'type': '1'
});

const options = {
    hostname: '1680660.com',
    port: 80,
    path: '/smallSix/findSmallSixHistory.do',
    method: 'POST',
    headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
        'Content-Length': Buffer.byteLength(lhc_query)
    }
};

const req = http.request(options, (res) => {
    console.log(`状态码: ${res.statusCode}`);
    console.log(`响应头: ${JSON.stringify(res.headers)}`);
    res.setEncoding('utf8');
    res.on('data', (chunk) => {
        console.log(`响应主体: ${chunk}`);
    });
    res.on('end', () => {
        console.log('响应中已无数据。');
    });
});

req.on('error', (e) => {
    console.error(`请求遇到问题: ${e.message}`);
});

// 写入数据到请求主体
req.write(lhc_query);
req.end();
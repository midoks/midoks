function benchmark(callback, times){
	if(typeof callback != 'function'){//回调函数判断
		if(this.browserName != 'IE')
			console.log('not function');
		else
			alert('not function');
	}
	if(typeof times != 'number'){
		if(this.browserName != 'IE')
			console.log('not number');
		else
			alert('not number');
	}
	var start = (new Date()).getTime();
	//循环执行
	for(var i=0; i<times; i++){
		callback();	
	}
	var end = (new Date()).getTime();
	var ResultStr = 'benchmark: time:'+(end-start)+'ms, times:'+times+' times;'+"\r\n";
	ResultStr += (end-start)/times+ 'ms/times'+"\r\n";
	if(this.browserName != 'IE')
		console.log(ResultStr);
	else
		return ResultStr;
}


/**
 *	@func 建立一个web服务器
 */

//加入http模块
var http = require("http");

//请求监听
/**
 *	@param request 请求信息
 *	@param response 响应信息
 */
//benchmark(
http.createServer(function(request, response){
	console.log(request.url);
	
	//响应头
	response.writeHead(200, {"Content-Type":"text/plain"});
	
	//响应内容
	response.write("hello world");

	//响应结束
	response.end();
}).listen(8888)//,1);
//console.log("end");



function Ajax(type,bool){

	var xhr = {};
	/*默认返回 html type 返回的类型为 */
	if(typeof(type)=='undefined'){
		xhr.type='HTML';
	}else{
		xhr.type=type.toUpperCase();
	}
	/*默认传输的方式 true 是异步 false 是同步 */
	if(typeof(bool)=='undefined'){
		xhr.async=true;
	}else{
		xhr.async=bool;
	}
	xhr.url = '';//网站地址
	xhr.send = '';//POST请求服务器地址? & 格式url;
	xhr.result = null;

	xhr.createXHR = function(){
		var request;
		try{ //判断浏览器是否原生态支持
			request = new XMLHttpRequest();
			if(request.overrideMimeType){
				request.overrideMimeType('text/html');
			}
		}catch(e){
			var v = ['Microsoft.XMLHTTP', 'MSXML.XMLHTTP', 'Microsoft.XMLHTTP',
				'Msxml2.XMLHTTP.7.0', 'Msxml2.XMLHTTP.6.0', 'Msxml2.XMLHTTP.5.0',
					'Msxml2.XMLHTTP.4.0', 'MSXML2.XMLHTTP.3.0', 'MSXML2.XMLHTTP'];
			for(var i=0;i<v.length;i++){
				try{
					request = new ActiveXObject(v[i]); 
					if(request){
						return request;
					}
				}catch(e){
					continue;
				}
			}
		}
		return request;
	}

	//换个名字
	xhr.XHR = xhr.createXHR();
	//进程控制
	xhr.processHandle = function(){
		if( xhr.XHR.readyState ==4 && xhr.XHR.status==200){
			if(xhr.type=='HTML'){
				xhr.result(xhr.XHR.responseText);
				return xhr.XHR.responseText;
			}else if(xhr.type=='JSON'){
				xhr.result(eval('('+xhr.XHR.responseText+')'));
				return eval('('+xhr.XHR.responseText+')');
			}else{
				xhr.result(xhr.XHR.responseXML);
				return xhr.XHR.responseXML;
			}
		}	
	};


	/**
	 *	get 获值
	 *	@param string url web文件
	 *	@param mixed result 数据操作
	 */
	xhr.get = function(url,result){
		xhr.url = url;
		if(result!=null){
			xhr.XHR.onreadystatechange = xhr.processHandle;
			xhr.result = result;
		}
		if(window.XMLHttpRequest){
			xhr.XHR.open('GET',xhr.url,xhr.async);
			xhr.XHR.send(null);
		}else{
			xhr.XHR.open('GET',xhr.url,xhr.async);
			xhr.XHR.send();
		}
	};

	/**
	 *	post 获值
	 *	@param string url web文件
	 *	@paramn mixed send 传向服务端的值
	 *	@param mixed result 数据操作
	 */
	xhr.post = function(url,send,result){
		// console.log(url,send,result);
		xhr.url = url;
		/* 分解过去的值 */
		if(typeof(send) == 'object'){
			var str = '';
			for(var pro in send){
				str +=pro +'='+send[pro]+'&';
			}
			xhr.send = str.substr(0,str.length-1);
		}else{
			xhr.send = send;
		}
		if(result!=null){
			xhr.XHR.onreadystatechange = xhr.processHandle;
			xhr.result = result;
		}
		xhr.XHR.open('POST',url,xhr.async);
		xhr.XHR.setRequestHeader('Content-type','application/x-www-form-urlencoded');//设置格式
		xhr.XHR.send(xhr.send);
	}
	return xhr;//返回
};

export default Ajax;
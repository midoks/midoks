//(function(){
/**
 * 	2013-5-11
 *	修改一个bug:添加发送ajax请求类型
 */



	/**
	 *	ajax的创建
	 *	@param string type 类型 传值时不分大小写 'html','json','xml'
	 *	@param boolean 类型的值,true表示异步传输方式,false表示同步传输方式,默认为true;
	 */
	function ajax(type,bool){
		var xhr = {};//创建一个对象
		
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

		//url值
		xhr.url = '';
		//post发送的数据
		xhr.sendString = '';
		
		/* 默认post get 返回的数据为null */
		xhr.returnval = null;
	
		//创建XMLHttpRequest;
		xhr.createXHR = function(){
			var request = false;
			//获取xhr对象
			try{request = new XMLHttpRequest();//非IE中
			}catch(e){
				//IE当中,各种测试
				var v = ['Microsoft.XMLHTTP', 'MSXML.XMLHTTP', 'Microsoft.XMLHTTP', 
 						 'Msxml2.XMLHTTP.7.0', 'Msxml2.XMLHTTP.6.0', 'Msxml2.XMLHTTP.5.0',
						 'Msxml2.XMLHTTP.4.0', 'MSXML2.XMLHTTP.3.0', 'MSXML2.XMLHTTP'];
				for(var i=0;i<v.length;i++){
					try{
						request = new ActiveXObject(v[i]);
						if(request){return request;}
					}catch(e){continue;}
				}
			}
			return request;
		}

		xhr.XHR = xhr.createXHR();
		//返回值控制
		xhr.controlXHR = function(){
			if(xhr.XHR.readyState==4 && xhr.XHR.status==200){
				if(xhr.type=='HTML'){
					xhr.returnval(xhr.XHR.responseText);
				}else if(xhr.type=='JSON'){
					xhr.returnval(eval('('+xhr.XHR.responseText+')'));
				}else if(xhr.type=='XML'){
					xhr.returnval(xhr.XHR.responseXML);
				}
			}
		}

		/**
		 *	get 方式获取值
		 *	@param string url 网站地址
		 *	@paran mixed result 结果数据
		 */
		xhr.get = function(url,result){
			if(result!=null){
				xhr.XHR.onreadystatechange = xhr.controlXHR;
				xhr.returnval = result;
			}
			//检查是否源生态支持XMLHttpRequest对象
			if(window.XMLHttpResquest){
				xhr.XHR.open('GET',url,xhr.async);
				xhr.XHR.send(null);
			}else{
				xhr.XHR.open('GET',url,xhr.async);
				xhr.XHR.send();
			}
		}

		/**
		 *	post 方式获取值
		 *	@param string url 网站地址
		 *	@param mixed sendString 发送的数据
		 *	@param mixed result 结果数据
		 */
		xhr.post = function(url,sendString,result){
			xhr.url = url;
			if(typeof(sendString)=='object'){
				var str='';
				for(var pro in sendString){
					str+=pro+'='+sendString[pro]+'&';
				}
				xhr.sendString = str.substr(0,str.length-1);
			}else{
				xhr.sendString = sendString;
			}
			//各种设置
			if(result!=null){
				xhr.XHR.onreadystatechange = xhr.controlXHR;
				xhr.returnval = result;
			}
			xhr.XHR.open('POST',url,xhr.async);
			xhr.XHR.setRequestHeader('request-type','ajax');//设置请求类型(是否是ajax请求)
			xhr.XHR.setRequestHeader('Content-type','application/x-www-form-urlencoded');//设置格式
			xhr.XHR.send(xhr.sendString);
		}
		return xhr;
	}

/*	ajax('html',true).post('test.php?QQ=ASD&BB=AA',null,function(data){
				alert(data);
			});	
*/
//})();

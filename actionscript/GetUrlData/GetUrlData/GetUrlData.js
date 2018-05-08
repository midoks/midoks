/**
 * @func 通过flash获取网页中数据
 * author:midoks
 */
//用匿名方法包裹
(function(){
/**
 * 	@func 通过FLASH 跨域获取数据 或 传输数据
 *	@param string Url 网站地址
 *	@param string Method 仅 GET POST (大小写不区分)
 */
function G(Url,Method){
	
	//声明一个对象
	var JsFlash = {};
	
	JsFlash.IsReady = false;//Flash是否初始化完毕 true 初始化完毕 | false 未初始化 | 默认false
	//JsFlash.fId = 'G_' + (Math.random().toString()).substr(2); 	//ID随机值
	JsFlash.fId = 'G_007'; 	//FLASH ID
	JsFlash.Id = 'GtUrl';	//给一个确认的值
	JsFlash.SwfPath = 'GetUrlData/GetUrlData.swf';				//swf路径

	JsFlash.init = function(){;//仅加载一次
		if(null!=document.getElementById(JsFlash.Id)){
			return true;
		}	
	}

	//数据保存
	JsFlash.resultBool = false;
	JsFlash.result = '';//保存结果
	
	JsFlash.error = new Array;//保存错误信息
	JsFlash.progress = new Array;//保存进程信息

	//测试数据保存
	JsFlash.test = '';

	//错误收集
	G.error = function(e){
		JsFlash.error.push(e);
		//console.log(JsFlash.error);
	}

	//数据完成加载
	G.result = function(e){JsFlash.result = e;};

	//进程加载
	G.progress = function(e){JsFlash.progress.push(e);};

	//flash准备好后,立即触发
	G.FlashTrigger = function(e){JsFlash.IsReady = e;};

	//测试用的
	G.test = function(e){
		JsFlash.test = e;
	}

	//创建div对象并插入BODY中
	function create_div(){
		var d = document.createElement('div');
		d.style.width=0;
		d.style.height=0;
		d.id = JsFlash.Id;
		return d;
	}

	//在HTML中插入FLASH
	function InitFlash(){
		if(JsFlash.init()){return true;}
		var html = new Array();
		var protocol = location.href.match(/^https/i) ? 'https://' : 'http://';//安全控制
		//宽高设置
		var fwidth = 0,fheight = 0;
		//嵌入网页
		html.push('<object classid="clsid:d27cdb6e-ae6d-11cf-96b8-444553540000" id="'+JsFlash.fId+'" codebase="'+protocol+'download.macromedia.com/pub/shockwave/cabs/flash/swflash.cab#version=9,0,0,0" width="'+fwidth+'" height="'+fheight+'" id="'+ JsFlash.fId +'" align="middle">');
		html.push('<param name="allowScriptAccess" value="always" ></param>');//允许脚本进入操作 如:ActionScript
		html.push('<param name="allowFullScreen" value="false" />');
		html.push('<param name="movie" value="'+JsFlash.SwfPath+'" />');
		html.push('<param name="loop" value="false" />');
		html.push('<param name="menu" value="false" />');
		html.push('<param name="quality" value="best" />');
		html.push('<param name="bgcolor" value="#ffffff" />');
		html.push('<param name="flashvars" value="id=123"/>');	//这个参数可以传值
		html.push('<param name="wmode" value="transparent"/>');
		html.push('<embed id="'+JsFlash.fId+'" src="'+JsFlash.SwfPath+'" loop="false" menu="false" quality="best" bgcolor="#ffffff" width="'+fwidth+'" height="'+fheight+'" name="'+JsFlash.fId+'" align="middle" allowScriptAccess="always" allowFullScreen="false" type="application/x-shockwave-flash" pluginspage="http://www.macromedia.com/go/getflashplayer" flashvars="id=123" wmode="transparent" />');
		html.push('</object>');
		//在IE或非IE中选择新插入	navigator.userAgent.match(/MSIN/)	//document.body.innerHTML = html.join('');
		var c_d = create_div();
		c_d.innerHTML = html.join('');
		document.body.appendChild(c_d);
	}

	//初始化flash
	JsFlash.InitFlash = InitFlash();
	
	//取得对象 @param string id id值
	function getObject(id){
		if (navigator.appName.indexOf("Microsoft") != -1) {
			return window[id];//非IE
		}else{
			return document[id];//IE下
		}
	}

	//直到JsFlash.IsReady为真
	//flash 加载延时
	function check_ready(bool){
		switch(bool){
			case false:
				//当flash表示自己准备好时,但是IE却不总是能如此...
				////错误的修复:不能扩展嵌入元素在Fire fox 中,必须使用传统的功能
				setTimeout(function(){check_ready(JsFlash.IsReady);},1); break;
			case true:	
				setTimeout(function(){
					get(JsFlash.fId,Url);
				},1);break;
			default:
				setTimeout(function(){
					get(JsFlash.fId,Url);
				},1);break;
		}
	}

	//获取资源
	function get(id,url){
		var t = getObject(id);
		t.get(url);	
		//资源获取或的操作
		var timer = setInterval(function(){
			if(JsFlash.result!=''){//console.log(JsFlash.result);
				clearInterval(timer);
				destory();
			}
		},10);
	}

	//销毁资源
	function destory(){
		var res = document.getElementById(JsFlash.Id);
		document.body.removeChild(res);
	}

	//xml字符串解析
	function xmlparse(data){
		var xmlobject;
		try{
			xmlobject = new ActiveXObject("Microsoft.XMLDOM");
			xmlobject.async = 'false';
			xmlobject.loadXML(data);
		}catch(e){
			var parser = new DOMParser();
			xmlobject = parser.parseFromString(data,'text/xml');
		}
		return xmlobject;
	}

	//返回数据的处理
	function datadel(type,func,data){
		switch(type){
			case 'json':func(eval('('+data+')'));break;
			case 'xml':func(xmlparse(data));break;
			default:func(data);
		}
	};
	
	/**
	 *	@param string DataType 返回的类型 默认是字符串  (都不区分大小分) | 以下是可选值
	 *  string
	 *  json
	 *  xml
	 *  @paran func 回调函数
	 */
	JsFlash.get = function(DataType,callback){
		var type = (DataType.toString()).toLowerCase();//数据类型
		//检查并获取数据
		check_ready(this.IsReady);
		var timer = setInterval(function(){
			if(JsFlash.result!=''){
				if(typeof callback == 'function'){//检查是否回调函数
					if('false'==JsFlash.result){
						alert('And allow the domain name in the goal-line, crossdomain.xml file access');
					}
					datadel(type,callback,JsFlash.result);
					//callback(JsFlash.result);
				};
				//console.log(JsFlash.test);
				clearInterval(timer);
			}
		},10);
	}

	return JsFlash;
}
//变为全局变量
window.G = G;
})();

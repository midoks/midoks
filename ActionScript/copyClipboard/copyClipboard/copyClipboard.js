(function(){



/**
 *	@func 把内容复制到剪贴板上
 */

function S(){
	
	var JsFlash = {};
	
	JsFlash.IsReady = false;//flash是否初始化
	JsFlash.compele = false;//复制是否完成
	JsFlash.SwfPath = 'copyClipboard/copyClipboard.swf';
	JsFlash.fId = 'C_' + (Math.random().toString()).substr(2);
	JsFlash.Id = 'c_007';
	//JsFlash.text = contentVal;

	
	//flash准备好后,立即触发
	S.FlashTrigger = function(e){JsFlash.IsReady = e;};
	//传回复制完成信号
	S.Compele = function(e){JsFlash.compele = e;console.log(JsFlash.compele);};
	
	//测试
	S.test = function(e){
		alert(e);
	}

	//创建div对象并插入BODY中
	function create_div(){
		var d = document.createElement('div');
		d.style.width = 0;
		d.style.height = 0;
		//d.style.background = 'blue';
		d.id = JsFlash.Id;
		return d;
	}

	//在HTML中插入FLASH
	function InitFlash(){
		//if(JsFlash.init()){return true;}
		var html = new Array();
		var protocol = location.href.match(/^https/i) ? 'https://' : 'http://';//安全控制
		//宽高设置
		var fwidth = 0,fheight = 0;
		//嵌入网页
		html.push('<object classid="clsid:d27cdb6e-ae6d-11cf-96b8-444553540000"  id="'+JsFlash.fId+'" codebase="'+protocol+'download.macromedia.com/pub/shockwave/cabs/flash/swflash.cab#version=9,0,0,0" width="'+fwidth+'" height="'+fheight+'" id="'+ JsFlash.fId +'" align="middle">');
		html.push('<param name="allowScriptAccess" value="always" ></param>');//允许脚本进入操作 如:ActionScript
		html.push('<param name="allowFullScreen" value="false" />');
		html.push('<param name="movie" value="'+JsFlash.SwfPath+'" />');
		html.push('<param name="loop" value="false" />');
		html.push('<param name="menu" value="false" />');
		html.push('<param name="quality" value="best" />');
		html.push('<param name="bgcolor" value="#ffffff" />');
		html.push('<param name="flashvars" value="id=123"/>');	//这个参数可以传值
		html.push('<param name="wmode" value="transparent"/>');
		html.push('<embed id="'+JsFlash.fId+'" src="'+JsFlash.SwfPath+'" loop="false" menu="false" quality="best" bgcolor="#ffffff" width="'+fwidth+'" height="'+fheight+'" name="'+JsFlash.fId+'" align="middle" allowScriptAccess="always" allowFullScreen="false" type="application/x-shockwave-flash" pluginspage="http://www.macromedia.com/go/getflashplayer" flashvars="" wmode="transparent" />');
		html.push('</object>');
		//在IE或非IE中选择新插入	navigator.userAgent.match(/MSIN/)	//document.body.innerHTML = html.join('');
		var c_d = create_div();
		c_d.innerHTML = html.join('');
		document.body.appendChild(c_d);
	}

	//JsFlash初始化
	JsFlash.init = InitFlash();
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
					setText(JsFlash.fId);
				},1);break;
			default:
				setTimeout(function(){
					setText(JsFlash.fId);
				},1);break;
		}
	}

	//销毁资源
	function destory(){
		var res = document.getElementById(JsFlash.Id);
		document.body.removeChild(res);
	}

	//重置
	function reset(){
		var timer = setInterval(function(){
			destory();
			clearInterval(timer);
		},100);
	}

	//获取资源
	function setText(id){
		var t = getObject(id);
		console.log(t);
		if(typeof JsFlash.text == 'string' ){
			t.set(JsFlash.text);
		}
	}

	JsFlash.set = function(e){
		JsFlash.text = e;
		check_ready(this.IsReady);
		console.log(JsFlash.compele+':完成的');
		var timer = setInterval(function(){
			//getObject(JsFlash.fId).click();
			//console.log(getObject(JsFlash.fId));
			if(JsFlash.compele){
				reset();
				clearInterval(timer);
			}
		},100);
	}

	return JsFlash;
////////////////////	
}


window.COPY = S;
})();

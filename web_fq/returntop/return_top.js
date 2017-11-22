//DOM加载完后执行
window.onload = function(){
	//获取class对象
	function get_class(ClassName){
		var tags = tags || document.getElementsByTagName('*');
		var list = [];
		for(var k in tags){
			var tag = tags[k];
			if(tag.className == ClassName){
				list.push(tag);
			}
		}
		return list;
	}

	//事件监听
	/**
	 *	比较通用型的事件监听
	 *	兼容IE5.0+IE6.0和 火狐
	 *	@param mixed elm  如 document.getElementById('body');
	 *	@param string evType 事件的类型
	 *	@param function fn 执行的方法
	 *	@param boolean useCapture (一般为false)
	 */
	function eventListener(elm, evType, fn, useCapture){
		if(elm.addEventListener){
			elm.addEventListener(evType, fn, useCapture);
			return true;
		}else if(elm.attachEvent){
			var r = elm.attachEvent('on'+evType,fn);
		}else{
			elm['on'+evType] = fn;
		}
	}

	var timer;
	//滑动回到顶部
	function sidebar(time, height){
		if(typeof time=='undefined'){
			var time = 50;
		}
		if(typeof height == 'undefined'){
			var height = 100;
		}


		//定时执行
		var timer = setInterval(function(){
			if(document.body.scrollTop==0){
				clearInterval(timer);
			}else{
				document.body.scrollTop = document.body.scrollTop - height;
			}
		
		},time);
	}

	var mtop =  get_class('midoks_returnTop');

	//监听并能点击回到顶部
	eventListener(mtop[0], 'click', function(){
		//document.body.scrollTop = 0;
		//sidebar(50,50);
		sidebar();
	},false);

	//监听鼠标移动事件
	eventListener(document.body, 'mousemove', function(){
		if(document.body.scrollTop > 35){
			mtop[0].style.display="block";
		}else{
			mtop[0].style.display="none";
		}
	},false);
}

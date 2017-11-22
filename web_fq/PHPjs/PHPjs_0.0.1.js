/*
	
   自己编写的JS库

*/
(function(){
	//申明一个自定义对象
	window['PHP'] = {};
	
	/*---------------------------------------
		@func $ 查找id标签 $
		@return object $ 返回这个元素的对象 $
	-----------------------------------------*/
	function $(){
		//传的参数
		var elements = new Array();
		for(var i=0;i<arguments.length;i++){
			var elements = arguments[i];
			if( typeof elements == 'string' ){
				element = document.getElementById(elements);
			}
			if(arguments.length==1){
				return element;
			}

		}
	}
	//加入全局
	window['PHP']['$']=$;
	
	
	/*---------------------------------------
		@func $ 获取时间值 $
		@return string $ 返回一个时间字串 $
	----------------------------------------*/
	function cztime(){
		var date = new Date();
		var _y = date.getFullYear();	//年的获取
		var _m = date.getMonth();   	//月的获取
		var _d = date.getDate();		//天的获取
		var _h = date.getHours();	//时的获取
		var _i = date.getMinutes();	//分的获取
		var _s = date.getSeconds();  //秒的获取
		if(_s<10){
			_s='0'+_s;
		}
		//拼装字串
		var str = _y+'-'+ _m + '-' + _d +' ' + _h + ':' + _i + ':' + _s;
		return str;
	}
	//加入全局
	window['PHP']['cztime']=cztime;

	/**
	 *	把不是字符串类型的,转换成字符串类型
	 *	@param string str 字符串
	 *	@param string
	 */
	function php_str(str){
		if(typeof(str)!='string'){
			return str = str.toString();
		}else{
			return str;
		}
	}

	/**
 	 *	字符串翻转
 	 *	@param string str 字符串
     *	@return string
  	 */
	function strrev(str){
		var str = php_str(str);
		var len = str.length-1;
		var t='',i;
		for(i=len;i>=0;i--){
			t += str[i];
		}
		return t;
	}
	window['PHP']['strrev']=strrev;

	/**
	 *	字符串长度
	 *	@param string str 字符串
	 *	@return num
	 */
	function strlen(str){
		var str = php_str(str);
		return str.length;
	}
	window['PHP']['strlen']=strlen;

	/**
	 *	字符串大写
	 *	@param string str 字符串
	 *	@return string
	 */
	function strtoupper(str){
		var str = php_str(str);
		return str.toUpperCase();
	}
	window['PHP']['strtoupper']=strtoupper;

	/**
	 *	字符串小写
	 *	@param string str 字符串
	 *	@return string
	 */
	function strtolower(str){
		var str = php_str(str);
		return str.toLowerCase();
	}
	window['PHP']['strtolower']=strtolower;

	/**
	 *	ord函数 返回字符串 string 第一个字符的 ASCII 码值
	 *	@param string str 字符
	 *	@return num	 ASCII 码值
	 */
	function ord(str){
		var str = php_str(str);
		return str.charCodeAt(0);
	}
	window['PHP']['ord']=ord;
	
	/**
	 *	implode函数
	 *	@param separator c 连接符
	 *	返回字符串值，其中包含了连接到一起的数组的所有元素，元素由指定的分隔符分隔开来。
	 *	@param array a 数组
	 *	return string 
	 */
	function implode(c,a){
		return a.join(c);
	}
	window['PHP']['implode']=implode;

	/**
	 *	rand 函数 产生随机数 默认产生0-1 之间的数 [ 0能得到 1不能得到 ]
	 *	@param num 参数1为最小值
	 *	@param num 参数2为最大值
	 *	在填写参数后,返回填写之间的正整数
	 *	没有填写参数,返回js的随机数
	 */
	function rand(){
		if(arguments.length>0){
			var e = new Array();
			for(var i=0;i<arguments.length;i++){
				e[i] = arguments[i];
			}
			if( !e[0] || !e[1]){
				var r = Math.random();
			}
		}else{
			return Math.random();
		}
	}
	window['PHP']['rand']=rand;

	/**
	 *	为了简化操作
	 *	就设置一个函数来控制cookie
	 *  函数cookie
	 *  @param string key key值
	 *  @param string value value值
	 *  @param num timeout 过期时间 默认1分钟,可不填 以秒为单位
	 */
	function cookie(){
		var e = new Array();
		var timeout = 600000;//60秒
		var today = new Date();
		for(var i=0;i<arguments.length;i++){
			e[i] = arguments[i]; 
		}	
		switch(e.length){
			case 1:
				var s = e[0] + '=';
				begin = document.cookie.indexOf(s);
				if(s!=-1){
					begin +=s.length;
					end = document.cookie.indexOf(';',begin);
					if(end==-1){
						end = document.cookie.length;
						return document.cookie.substr(begin,end)!='undefined'?document.cookie.substr(begin,end):'';
					}
				}break;
			case 2:	
				today.setTime(today.getTime() + timeout);
				return document.cookie = e[0]+'='+e[1]+';expires='+(today.toLocaleString());break;
			case 3:
				today.setTime(today.getTime() + e[2]*1000);
				return document.cookie = e[0]+'='+e[1]+';expires='+(today.toLocaleString());break;
			default:
				return false;break;	
		}
	}
	window['PHP']['cookie']=cookie;

})();

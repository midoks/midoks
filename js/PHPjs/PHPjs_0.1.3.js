/*
 * 自己编写的JS库
 * midoks
 * s:2012.12.6
 * 把JS编写成PHP函数的形式,并不推荐大家大量的使用写,而是为了学习JS。
 * 希望大家,能看到我所写的这个PHPjs,大家能更好的学习JS。
 */
(function(){
	//申明一个自定义对象
	var PHP = {};
	PHP.returnval=null;//返回值
/**-------------------------------------------
 *											*		
 *				数字处理					*
 *											*
 -------------------------------------------*/
	/**
	 *	@func 绝对值
	 *	@param int math 整数
	 */
	PHP.abs = function(math){return Math.abs(math)};

	/**
	 * @func pi值
	 */
	PHP.pi = Math.PI;

	/**
	 *	@func 对浮点数进行四舍五入
	 *	@param int math 整数
	 */
	PHP.round = function(math){return Math.round(math);};

	/**
	 *	@func 向上取整
	 *	@param int math 整数
	 */
	PHP.ceil = function(math){return Math.ceil(math);};

	/**
	 *	@func 向下取整
	 *	@param int math 整数
	 */
	PHP.floor = function(math){return Math.floor(math);};

	/**
	 *	rand 函数 产生随机数 默认产生0-1 之间的数 [ 0能得到 1不能得到 ]
	 *	@param int min 参数1为最小值
	 *	@param int max 参数2为最大值
	 *	@param int deep 精确度
	 *	在填写参数后,返回填写之间[并包括MAX和MIN]的正整数
	 *	没有填写参数,返回js的随机数
	 */
	PHP.rand = function(min,max,deep){
		if(typeof deep == 'undefined'){
			var deep=3;
		}
		if(arguments.length>1){
			var abs = max-min;//绝对值
			var r = Math.random();//随机数
			var s = abs * r;//随机的总数
			var v = s.toString();//标准比较 以0.5为隔
			var z = Math.pow(10,deep-1)*5;//以0.5为隔[表现的形式]
			if(v.substr(2,deep)>z){
				return min+Math.floor(s)+1;
			}else{
				return min+Math.floor(s);
			}	
		}else{return Math.random();}
	};

	/**
	 *	max 函数 找出最大值
	 *	@param int 整数
	 *	...可以是多个整数
	 *	@return int 最大值
	 */
	PHP.max = function(){
		var i = arguments,
			//定义一个临时变量
			m=arguments[i];
		if(i.length==0){
			return false;
		}else if(i.length==1){
			return false;
		}
		for(var p=0;p<i.length-1;p++){
			if(i[p]>i[p+1] && i[p]>m){
				m = i[p];
			}else{
				m = i[p+1]>m ? i[p+1] : m;
			}
		}
		return m;
	};

	/**
	 *	min 函数 找出最小值
	 *	@param int 整数
	 *	...可以是多个整数
	 *	@return int 最小值
	 */
	PHP.min = function(){
		var i = arguments,
			//定义一个临时变量
			m=arguments[0];
		if(i.length==0){
			return false;
		}else if(i.length==1){
			return false;
		}
		for(var p=0;p<i.length-1;p++){
			if(i[p]<i[p+1] && i[p]<m){
				m = i[p];
			}else{
				m = i[p+1]<m ? i[p+1] : m;
			}
		}
		return m;
	};

	//sin cos 等函数
	
	/**
	 *	sin 函数
	 *	@param int x 整数
	 *	return sin 值
	 */
	PHP.sin = function(x){return Math.sin(x);};

	/**
	 *	cos 函数
	 *	@param int x 整数
	 *	return cos 值
	 */
	PHP.cos = function(x){return Math.cos(x);};

	/**
	 *	tan 函数
	 *	@param int x 整数
	 *	return tan 值
	 */
	PHP.tan = function(x){return Math.tan(x);};

	/**
	 *	asin 函数
	 *	@param int x 整数
	 *	return tan 值
	 */
	PHP.asin = function(x){return Math.asin(x);};

	/**
	 *	acos 函数
	 *	@param int x 整数
	 *	return acos 值
	 */
	PHP.acos = function(x){return Math.acos(x);};

	/**
	 *	atan 函数
	 *	@param int x 整数
	 *	return atan 值
	 */
	PHP.atan = function(x){return Math.atan(x);};

	/**
	 *	atan2 函数 两个参数的反正切
	 *	@param number x 整数
	 *  @param number y 整数
	 *	return atan2 值
	 */
	PHP.atan2 = function(x,y){return Math.atan2(x,y);};

	//科学计数
	
	/* E 常量 */
	PHP.E = Math.E;

	/**
	 *	log 函数 自然对数
	 *	@param int float 数字
	 *	return 自然对数
	 */
	PHP.log = function(x){return Math.log(x);};

	/**
	 *	pow 指数表达式
	 *	@param number 基数
	 *	@param number 表达式
	 *	@return number
	 */	
	PHP.pow = function(base,exp){return Math.pow(base,exp);};

	/**
	 *	exp 计算E的指数
	 *	@param number 多少次方
	 *	@return E的x次方
	 */	
	PHP.exp = function(x){return Math.exp(x);};

	/**
	 *	sqrt 开平方根
	 *	@param number arg
	 *	@return number
	 */
	PHP.sqrt = function(x){return Math.sqrt(x);};

/**-------------------------------------------
 *											*
 *				时间函数处理				*
 *											*
 -------------------------------------------*/
	/**
	 *	返回当前的 Unix 时间戳
	 *	精确到秒级
	 */
	var timedel = new Date;//时间函数声明
	PHP.time = function(){
		var t = timedel.getTime().toString();
		return t.substr(0,10);
	};

	/**
	 *	返回当前的 Unix 时间戳 和 微秒数
	 */
	PHP.microtime = function(){
		var t = timedel.getTime().toString();
		return [t.substr(0,10),t.substr(10,3)];
	};

/**------------------------------------------
 *											*
 *				字符串处理					*
 *											*
 -------------------------------------------*/

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
	};
	
	/**
	 * @func 截取字符串
	 * @param string str
	 * @param int start 开始位置
	 * @param int len 截取长度
	 */
	PHP.substr = function(str,start,len){
		var str = php_str(str);
		return str.substr(start,len);
	}

	/**
 	 *	字符串翻转
 	 *	@param string str 字符串
     *	@return string
  	 */
	PHP.strrev = function(str){
		var str = php_str(str);
		var len = str.length-1;
		var t='',i;
		for(i=len;i>=0;i--){t += str[i];}
		return t;
	};

	/**
	 *	字符串长度
	 *	@param string str 字符串
	 *	@return num
	 */
	PHP.strlen = function(str){
		var str = php_str(str);
		return str.length;
	};

	/**
	 *	字符串大写
	 *	@param string str 字符串
	 *	@return string
	 */
	PHP.strtoupper = function(str){
		var str = php_str(str);
		return str.toUpperCase();
	};

	/**
	 *	字符串小写
	 *	@param string str 字符串
	 *	@return string
	 */
	PHP.strtolower = function(str){
		var str = php_str(str);
		return str.toLowerCase();
	};
	
	/**
	 *	str_repeat函数重复一个字符串
	 *	@param string $str 字符串
	 *	@param int $num 重复的次数
	 */
	PHP.str_repeat = function(str,num){
		if(num<0){return '';
		}else{
			/*var t='';
			for(var i=0;i<=num;i++){
				t += str;
			}下面为优化的*/
			var t = new Array();
			for(var i=0;i<num;i++){
				t[i]=str;
			}
			return t.join('');
		}
	};

	/**
	 *	strpos 函数 查找字符串首次出现的位置
	 *	@param string str 字符串
	 *	@param string find 查询的字符
	 *	@return int 返回查找字符串首次出现位置
	 */
	PHP.strpos = PHP.strstr = PHP.strchr = function(str,find){
		return str.indexOf(find);
	}

	/**
	 *	strrpos 函数 查找字符串最后出现的位置
	 *	@param string str 字符串
	 *	@param string find 查询的字符
	 *	@return int 返回查找字符串首次出现位置
	 */
	PHP.strrpos = function(str,find){
		if(find.length>1){
			find = find.substr(find.length-1,1);
		}
		return str.lastIndexOf(find);
	}

	/**
	 *	字符串替换
	 *	@param mixed search 字符串或数组
	 *	@param mixed replace 字符串或数组
	 *	@param mixed subject 字符串或数组
	 *	@param int count 替换的次数   ---暂时未实现
	 *	@return string 返回字符串
	 */
	PHP.str_replace = function(search,replace,subject){
		str = php_str(subject);
		return str.replace(search,replace);
	}

	/**
	 *	将字符串转化为数组
	 *	@param string str 字符串
	 *	@param int num 每一段的长度
	 *	@return array 返回数组
	 */
	PHP.str_split = function(str,num){
		if(typeof num != 'number'){var num=1;}
		var arr = new Array;
		for(var i=0,p=0;i<str.length;i+=num){
			arr[p] = str.substr(i,num);p++;
		}
		return arr;
	}
	
	/**
	 *	@func 随机将字符串打乱
	 *	@param string str 字符串
	 *	@return string 返回打乱的字符串
	 */
	PHP.str_shuffle = function(str){
		var arr = new Array;//创建数组
		arr = str.split('');
		var rd = function(min,max){
			var deep=3;
			var abs = max-min;//绝对值
			var r = Math.random();//随机数
			var s = abs * r;//随机的总数
			var v = s.toString();//标准比较 以0.5为隔
			var z = Math.pow(10,deep-1)*5;//以0.5为隔[表现的形式]
			if(v.substr(2,deep)>z){return min+Math.floor(s)+1;}else{
				return min+Math.floor(s);
			}	
		};var t='';//返回的值,也是临时值
		for(var i=0;i<arr.length;){
			if(arr.length==1){ return t+=arr[0];}
			var rdom = rd(0,arr.length-1);//取随机数
			t +=arr.splice(rdom,1);
		}return t;
	}

	/**
	 * str_pad — 使用另一个字符串填充字符串为指定长度
	 * @param string input 输入的字符串
	 * @param int string 指定长度
	 * @param string pad_str 填充的字符串 默认为空格
	 * @param int type 填充的方式 0:right(默认) 1:left 2:both 
	 * 	如果是选择2,指定的长度减input的长度为奇数,会舍去小数的。
	 * @return string 放回填充后的字符串
	 */
	PHP.str_pad = function(input,length,pad_str,type){
		var str = php_str(input);
		if( typeof length ==='undefined' || length<=str.length){
			return str;
		}else{	
			var len = length-str.length;
			if(len<=0) return str;
			var n = new Array();
			pad_str = typeof pad_str === 'undefined' ? ' ' : pad_str;
			for(var i=0;i<len;i++){n[i] = pad_str;}
			if(typeof type === 'undefined'){
				t = n.join('').substr(0,len);return str + t;
			}
			//所有的参数都填写的情况下
			t = n.join('').substr(0,len);//console.log(t);
			switch(type){
				case 0: return str + t;break;
				case 1: return t+str;break;
				case 2: z = n.join('').substr(0,len/2);return z+str+z;break;
				default:return str;break;
			}
		}
	};

	/**
	 *	trim 函数 取出首尾的空格字符和其它一些字符
	 *	@param string str 字符串
	 *	@param string charlist 首尾要除去的字符
	 *	@return string 返回除去后的字符串
	 */
	PHP.trim = function(str,charlist){
		if(typeof charlist==='undefined'){
			charlist = /^[\s\r\n\t\0\x0B]*(.*[^\s\r\n\t\0\x0B])?[\s\r\n\t\0\x0B]*/ig;
			return str.replace(charlist,"$1");
		}else{
			charlist = '/['+charlist+']*(.*[^'+ charlist +'])?['+charlist+']*/ig';
			charlist = eval(charlist);
			return str.replace(charlist,"$1");
		}
	};

	/**
	 *	ord函数 返回字符串 string 第一个字符的 ASCII 码值
	 *	@param string str 字符
	 *	@return int	 ASCII 码值
	 */
	PHP.ord = function(str){
		var str = php_str(str);
		return str.charCodeAt(0);
	};

	/**
	 * chr函数,返回指定的字符
	 * @param int num 整数	'78,79,9';
	 * @return char 返回字符
	 */
	PHP.chr = function(num){
		var type = typeof num;
		if( type ==='number' ){
			return String.fromCharCode(num);
		}else if(type == 'string'){
			arr = num.split(',');
			for(var i=0;i<arr.length;i++){
				arr[i] = String.fromCharCode(arr[i]);
			}
			return arr.join('');
		}
	};

	/**	
	 *	bin2hex 将二进制数据转换成十六进制表示
	 *	@param mixed str 字符串 | 数字
	 *	@return 返回ASCII字符串,为参数str的十六进制表示.
	 *	note:计算位数问题:php跟服务器环境有关,javascript则是固定64位
	 */
	PHP.bin2hex = function(str){	
		if(typeof str !='number'){
			var t = new Array;
			for(var i=0;i<str.length;i++){
				var e=str.charCodeAt(i);
				t[i] = e.toString(16);
			}
			return t.join('');
		}else{
			return str.toString(16);
		}
	}

	/**
	 *	wordwrap 打断字符串为指定数量的字符
	 *	@param string str 字符串
	 *	@param int width 以多少字符为打断 默认宽度1
	 *	@param string sep 分隔符 默认 '\n'
	 *	@param boolean cut 0 默认 字符串 1 数组
	 *	@return string 字符串
	 */
	PHP.wordwrap = function(str,width,sep,cut){
		var s = str.length;//保存字符的长度
		if(typeof width=='undefined'){width=1;}
		if(typeof sep =='undefined'){sep ='\n';}
		if(typeof cut=='undefined'){cut=0;}	
		if(cut==0){
			var string = '';
			for(var i=0;i<s;i+=width){
				if((i+width)>s){
					string += str.substring(i,i+width);
					return string;
				}
				string += str.substring(i,i+width) + sep;
			}
		}else{
			var string = new Array();
			var t=0;
			for(var i=0;i<s;i+=width){
				if((i+width)>s){
					string [t]= str.substring(i,i+width);
					return string;
				}
				string [t]= str.substring(i,i+width);
				t++;
			}
		}
		return string;
	}

	//关于正则配置
	/**
	 *	preg_match 函数 匹配一次就会停止匹配了。
	 *	@param string match 匹配表达式
	 *	@param string subject 对象
	 *	@return array 返回匹配的数组
	 */
	PHP.preg_match = function(match,subject,arr){
		var t = subject.match(eval(match));
		if(typeof arr =='function'){arr(t);}
		return t;
	}

	/**
	 *	preg_match_all 函数 preg_match 也能实现,主要有人习惯了.
	 *	@param string match 匹配表达式
	 *	@param string subject 对象
	 *	@param array 返回的数据,可操作
	 *	@return array 返回匹配的数组
	 */
	PHP.preg_match_all = function(match,subject,arr){
		t = subject.match(eval('/'+match+'/g'));
		if(typeof arr =='function'){arr(t);}
		return t;
	}

	/**
	 *	preg_replace 执行一个正则表达式的搜索和替换
	 *	@param mixed pattern 正则表达式
	 *	@param mixed replacement 替换
	 *	@param mixed subject 对象
	 *	[@param function c 回调操作] 
	 *	@return 如果subject是数组则返回数组,其他返回字符串
	 */
	PHP.preg_replace = function(pattern,replacement,subject,c){
		pt = eval('/'+pattern+'/g');//正则
		if(typeof subject == 'object'){
			var sarr = new Array;
			for(var i=0;i<subject.length;i++){
				sarr[i] = subject[i].replace(pt,replacement);
			}	
			if(typeof c == 'function'){c(sarr);}
			return sarr;
		}else if(typeof subject == 'string'){
			p = subject.replace(pt,replacement);
			if(typeof c == 'function'){c(p);}
			return p;
		}
		
	}



/**------------------------------------------
 *											*
 *				数组处理					*
 *											*
 -------------------------------------------*/

	/**
	 *	implode函数
	 *	@param separator c 连接符
	 *	返回字符串值，其中包含了连接到一起的数组的所有元素，元素由指定的分隔符分隔开来。
	 *	@param array a 数组
	 *	return string 
	 */
	PHP.implode = function(c,a){
		return a.join(c);
	};

	/**
	 *	explode 函数
	 *  使用一个字符串分割另一个字符串
	 *  @param string separator 分割符
	 *  @param string str 字符串
	 *  @param int limit 分割的次数
	 *  @return array 返回数组
	 */
	PHP.explode = function(separator,str,limit){
		if(typeof limit == 'undefined'){
			return str.split(separator);
		}
		return str.split(separator,limit);
	}
	/**
	 *	array_reverse 函数
	 *	返回一个单元顺序相反的数组
	 *	@param &array arr
	 */
	PHP.array_reverse = function(arr){
		if(typeof arr =='object' || typeof arr =='array'){
			return arr.reverse();
		}else{
			return arr;
		}
	}
	
	/**
	 *	array_shift 函数
	 *	将数组开头的单元移出数组
	 *	@param &array arr 数组
	 *	@return array 插入的
	 */
	PHP.array_shift = function(arr){
		if(typeof arr =='object' || typeof arr =='array'){
			return arr.shift();	
		}else{
			return arr;
		}
	}

	/**
	 * array_unshift 函数
	 * 将数组开头插入一个或对个单元
	 * @param &array arr 数组
	 * @param array carr 插入单元
	 * @return array 返回插入的单元
	 */
	PHP.array_unshift = function(arr,carr){
		if(arguments.length>2){//如果大于2,说明多个元素压入
			var r = new Array;
			for(var i=1;i<arguments.length;i++){	
				r[i-1] = arguments[i];
			}
		}
		if(typeof arr =='object' || typeof arr == 'array'){
			if(typeof r =='object' || typeof r == 'array'){
				for(var i=0;i<r.length;i++){
					if(i==(r.length-1)){	
						return arr.unshift(r[i]);
					}
					arr.unshift(r[i]);				
				}
				return arr;
			}
			return arr.unshift(carr);
		}else{
			return arr;
		}
	
	}

	/**
	 * array_pop 函数 
	 * 将最后一个元素弹出
	 * @param array arr 数组
	 * @return 返回弹出的元素
	 */
	PHP.array_pop = function(arr){
		if(typeof arr =='object' || typeof arr =='array'){
			return arr.pop();
		}else{
			return arr;
		}
	};

	/** 
	 * array_push 函数
	 * 将一个或对个值,压入末尾
	 * @param array arr 数组
	 * @param mixed e 一个或对个元素
	 * @return 返回压入后的元素
	 */
	PHP.array_push = function(arr,e){
		if(arguments.length>2){//如果大于2,说明多个元素压入
			var r = new Array;
			for(var i=1;i<arguments.length;i++){	
				r[i-1] = arguments[i];
			}
		}
		if(typeof arr =='object' || typeof arr == 'array'){
			if(typeof r =='object' || typeof r == 'array'){
				for(var i=0;i<r.length;i++){
					if(i==(r.length-1)){	
						return arr.push(r[i]);
					}
					arr.push(r[i]);				
				}
				return arr;
			}
			return arr.push(e);
		}else{
			return arr;
		}
	}

	/**
	 *	array_merge  合并一个或多个数组
	 *	如果不是数组类型 不返回任何值
	 *	@param array 第一个数组
	 *	@param array 第二个数据
	 *	@return array 返回的数据
	 */
	PHP.array_merge = function(){
		var arr = new Array;
		var timer = 0;
		for(var i=0;i<arguments.length;++i){
			for(var j=0;j<arguments[i].length;++j){
				arr[timer] = arguments[i][j];
				++timer;
			}
		}
		return arr;
	};

	/**
	 *	为了简化操作
	 *	就设置一个函数来控制cookie
	 *  函数cookie
	 *  @param string key key值
	 *  @param string value value值
	 *  @param num timeout 过期时间 默认1分钟,可不填 以秒为单位
	 */
	 PHP.cookie = function(){
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
	};

/**-----------------------------------
 *									*	
 *			文件的操作				*
 *									*
 -----------------------------------*/
//	NOTE:在JS的文件的操作，并不是真正的文件操作。
//	而是使用ajax，获取网页的数据，并进行操作。
	
   /**
	*  借鉴:高洛峰 Ajax3.0
	*  为文件获取文件过去,操作ajax对象。
	*  @param string type HTML、XML和JSON,默认HTML,传值是不区分大小写
	*  @param boolean bool  true表示异步传输，false表示同步传输
	*/
	function php_ajax(type,bool){
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
		xhr.result=null;

		xhr.createXHR = function(){
			try{//判断浏览器是否原生态支持
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
						if(request){return request;}
					}catch(e){continue;
					
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
		 *	get 获值
		 *	@param string url web文件
		 *	@paramn mixed send 传向服务端的值
		 *	@param mixed result 数据操作
		 */
		xhr.post = function(url,send,result){
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

	/**
	 *	判断是否是方法并把第二参数的数据给它
	 *	@param function func 方法
	 *	@param mixd data 数据
	 *	@return 没有返回值
	 */
	function is_function(func,data){if(typeof func == 'function'){func(data);};};

	/**
	 *	创建标签
	 *	@param string elm 标签名
	 *	@return bom对象
	 */
	function create_elm(elm){
		var f;
		try{f = document.createElement(elm);//非IE下
		}catch(e){f = document.createElement(eval('<'+elm+'></'+elm+'>'));//IE下 
		}
		return f;
	};
	//因为ajax不能跨域操作的,而想的解决方法
	/**
	 *	@param string url 网站的地址
	 *	@return string 返回DOM
	 */
	function create_iframe(url){
		var f = create_elm('iframe');;
	    f.style.display='none';
		f.src=url;
		f.id='PHPjs';
		//在网页要先显示
		document.body.appendChild(f);
		return document.getElementById('PHPjs');
		//在网页上,不显示[试过,不行]
		return f;
	};

	/**
	 *	@param string url 网站的地址
	 *	@param mixed data 数据控制
	 *	@param boolean show 是否显示
	 */
	function get_iframe(url,data,show){
		var sign = create_iframe(url);
		//加载完后,触发
		sign.onload = function(){
			var end =  sign.contentDocument;//console.log(sign.contentDocument.getElementById('title'));
			is_function(data,end);
			if(show){document.body.removeChild(sign);}
			return end;
		}		
	};

	/**
	 *	创建script头方式
	 *	@param string url 网站地址
	 *	@return 返回标签对象
	 */
	function create_script(url){
		var f = create_elm('script');
		f.type = 'text/javascript';
		f.src=url;
		document.head.appendChild(f);
		return f;
	}
	/**
	 *	获取数据
	 *	@param string url 网站地址
	 *	@param mixed data 回调函数
	 *	@return 无返回值
	 */
	function get_script(url,data){
		//对url处理
		var c = url.split('?');
		var p = c[1].split('&');
		var name ='PHPjs';
		var r = name + '_' + Math.random().toString().substr(2);//随机
		//组装url
		var zurl = c[0] + '?' + p.join('&') + '&' + name + '=' + r;
		window[r] = data;
		var f = create_script(zurl);
		//获得数据后,回收资源
		f.onload = function(){
			try{delete window[r];} catch(e){}//删除全局变量
			document.head.removeChild(f);
		}
	}

   /**
	* file_get_contents | file 函数 获取的网页的数据
	* @param string url 网站的地址
	* @param mixed data 对返回的值,操作
	* @param boolean show iframe是显示在页面上 默认不显示[false]。也就意为没有返回值了。[true],有返回值
	* @return mied 获取网页的数据
	*/
	PHP.file_get_contents = PHP.file = function(url,data,show){
		var localDomain = document.domain;//当前域
		if(typeof show == 'undefined'){show=false;}
		//三种选择
		//1 同域的情况下
		var matchDomain = /(http:|https:)?\/\/(\w*\.\w*\.\w{2,3}[^\/])\//;//提出传来的域名
		var fromDomain = matchDomain.exec(url);
		/* 直接调用本域下文件的 */
		if(!fromDomain){
			return php_ajax('HTML',true).get(url,data);
		}
		if(localDomain == fromDomain[2]){
			return php_ajax('HTML',true).get(url,data);
		}else if((function(localDomain,fromDomain){
			var MRole = /(.*\.)?(.*\.\w{2,3})/,//匹配域名的规则
			match=false;
			var murl = MRole.exec(fromDomain[2]);
			var lurl = MRole.exec(localDomain);
			if(murl.pop()==lurl.pop()){return true;}
			return match;
		})(localDomain,fromDomain)){
		//2 在与子域的情况下[使用iframe解决] 需要的把iframe网页里设置成 例如:phpjs.com 才能使用
		//使用getElementById(),getElementByTagName();设置
			var MRole = /(.*\.)?(.*\.\w{2,3})/;//匹配域名的规则
			lurl = MRole.exec(localDomain).pop();
			document.domain = lurl;//console.log(lurl);
			var d = get_iframe(url,data,show);
			return d;
		}else{
		//3 获取其他域的情况下[使用iframe解决] 需要的把iframe网页里设置成 例如:phpjs.com 才能使用
		// 使用其他域的时候,意味你要是脚本语言如PHP,JSP,...请在头消息中设置是text/javascript这样才能更好使用
		// 使用方式与jquery json 一样
			var d = get_script(url,data);
			return d;
		}
	}
/**----------------------------------
 *				json处理			*
 -----------------------------------*/
	/**
	 *	json编码
	 *	@param array arr js数组[js只有索引数组]
	 *	@return string 返回字符串json数据
	 */
	function php_json_encode_object(object){//对单一对象的编码
		var strjson;//保存字符串数据
		//if(typeof object == 'object'){
		strjson = '{';
		for(l in object){
			strjson += '"' + l + '":"' + object[l] + '",';
		}
		strjson += '}';
		//}
		console.log(strjson);
		return strjson;
	}
	function php_json_encode_multiobject(mobject){//对多层对象的编码
		var str = '{';//声明一个字符串并赋值
		for(var o in mobject){
			if(typeof mobject[o] != 'object'){
				str += '"' + o + '":"' + mobject[o] + '",';
			}else{
				str += '"'+ o +'":' + php_json_encode_multiobject(mobject[o]) + ',';
			}
		}
		str +='}';
		return str;
	}
	PHP.json_encode = function(arr){
		return php_json_encode_multiobject(arr);
	}

	/**
	 *	json解密
	 *	@param string str 字符串
	 *	@return 返回对象数据
	 */
	PHP.json_decode = function(str){
		var data = eval('('+str+')');	
		return data;
	}
/**----------------------------------
 *				URL编码				*
 -----------------------------------*/
	
	/**
	 *  @bug:不能处理中文字
	 *	@func base64_encode 加密
	 *	@param string str 字符串
	 *  @return 返回加密后的字符串
	 */
	PHP.base64_encode = function(str){
		var b64_encode = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
		var out,i,len;
		var c1,c2,c3;
		len = str.length;
		i = 0;
		out = '';
		while(i<len){
			c1 = str.charCodeAt(i++) & 0xFF;//控制在8位内
			if(i==len){
				out += b64_encode.charAt(c1>>2);
				out += b64_encode.charAt((c1 & 0x3) << 4);
				out += '==';
				break;
			}
			c2 = str.charCodeAt(i++) & 0xFF;//控制在8位内
			if(i==len){
				out += b64_encode.charAt(c1>>2);
				out += b64_encode.charAt(((c1 & 0x3) << 4) | ((c2 & 0xF0)>>4));
				out += b64_encode.charAt((c2 & 0xF)<<2);
				out += '=';
				break;
			}
			c3 = str.charCodeAt(i++) & 0xFF;//控制在8位内
			out += b64_encode.charAt(c1>>2);
			out += b64_encode.charAt(((c1&0x3)<<4) | ((c2&0xF0)>>4));
			out += b64_encode.charAt(((c2&0xF)<<2) | ((c3&0xC0)>>6));
			out += b64_encode.charAt(c3&0x3F);		
		}
		return out;
	}

	/**
	 * @bug:不能处理中文字
	 * @func base64 解密
	 * @param string str 解密字符串
	 * @return 返回解密后的字符串
	 */
	PHP.base64_decode = function(str){
		var b64_decode = new Array(
			-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
    		-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
    		-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63, 
    		52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, 
    		-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 
    		15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, 
    		-1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 
    		41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1	
		);
		var c1,c2,c3,c4;
		var i,len,out;
		i = 0;len = str.length;out = '';//初始化值
		while(i<len){
			/* c1操作 */
			do{
				c1 = b64_decode[str.charCodeAt(i++) & 0xFF];		
			}while(i<len && c1==-1);
			if(c1==-1){
				break;
			}
			/* c2操作 */
			do{
				c2 = b64_decode[str.charCodeAt(i++) & 0xFF];		
			}while(i<len && c2 ==-1);
			if(c2 == -1){
				break;
			}	
			out += String.fromCharCode((c1<<2) | (c2 & 0x30) >>4);
			/* c3操作 */ 
			do{
				c3 = str.charCodeAt(i++) & 0xFF;
				if(c4==61){
					return out;
				}
				c3 = b64_decode[c3];
			}while(i<len && c3 == -1);
			if(c3 == -1){
				break;
			}
			out += String.fromCharCode(((c2 & 0xF) << 4) | (c3 & 0x3C)>>2);
			/* c4操作 */
			do{
				c4 = str.charCodeAt(i++) & 0xFF;
				if(c4==61){
					return out;
				}
				c4 = b64_decode[c4];
			}while(i<len && c4 == -1);
		
			if(c4 == -1){
				break;
			}
			out += String.fromCharCode(((c3&0x03)<<6)|c4);
		}
		return out;
	}

	/**
	 *  @url http://www.blogjava.net/hadeslee/archive/2007/11/16/160544.html 参考地址
	 *	@func urlencode 编码
	 *	@param string url URL地址
	 *	@return 返回编码后地址
	 */
	PHP.urlencode = function(url){
		if(url==null || url==''){return '';}
		var NewUrl = '';
		function toupper(str){return str.toString(16).toUpperCase();}
		for(var i=0, icode, len=url.length; i<len; i++){
			icode = url.charCodeAt(i);//转化成对应的ASCII码值
			if(icode<0x0f){
				NewUrl += '%0' + icode.toString(16).toUpperCase();
			}else if(icode<0x80){
				if(icode==0x20){
					NewUrl += '+';//空格
				}else if((icode>=0x30 && icode <=0x39) || (icode>=0x41 && icode <=0x5A) || (icode>=0x61 && icode <=0x7A)){//数字和字母区间
					NewUrl += url.charAt(i);//charAt 返回索引位置;
				}else{
					NewUrl += '%' + toupper(icode);//符号的区间
				}
			}else if(icode<0x7ff){
				NewUrl += '%' + toupper(0xC0 + (icode>>6));
				NewUrl += '%' + toupper(0x80 + icode%0x40);
			}else{
				//中文encodeURI()编码
				NewUrl += '%' + toupper(0xE0 + (icode>>12));
				NewUrl += '%' + toupper(0x80 + (icode>>6)%0x40);
				NewUrl += '%' + toupper(0x80 + icode%0x40);
			}
		}
		return NewUrl;
	}

	/**
	 *  @time 2013-1-31 解决urlencode中文编码
	 *  @func urldecode 解码
	 *	@param string url URL编码后地址
	 *	@return 返回解码后地址
	 */
	PHP.urldecode = function(url){
		var NewUrl = '';
		var len = url.length;
		var hanzi = '';//接受汉字
		for(var i=0,icode;i<len;i++){
			icode = url.charCodeAt(i);//转化成对应的ASCII码值
			if(url[i] == '+'){//遇到+,返回空
				NewUrl += ' ';
			}else if((icode>=0x30 && icode <=0x39) || (icode>=0x41 && icode <=0x5A) || (icode>=0x61 && icode <=0x7A)){
				NewUrl += url[i];//字母和数字不变
			}else if(url[i] == '%'){
				var t ="0x" + url.substr(i+1,2);
				if(eval(t)>127){//不在ascii内,为汉字
					//中文encodeURI()解码
					hanzi = url.substr(i,9);
					var hword = decodeURI(hanzi);
					NewUrl += hword;
					i+=8;
				}else{
					NewUrl += String.fromCharCode(t);//特殊符号
					i+=2;
				}
		
			}else{}
		}
		return NewUrl;
	}

/**-----------------------------------
 *				杂项处理			*
 -----------------------------------*/
	/**
	 *	sleep函数
	 *	@param func callback 休眠后要执行的函数
	 *	@param int time 要休眠的时间[毫秒][大概的范围内]
	 *	@return 无返回值
	 */
	PHP.sleep = function(callback,time){
		setTimeout(callback,time);
	};

	/**
	 *	eval函数 js自带的
	 *	@param mixed c 要执行的代码
	 */
	PHP.eval = function(c){return eval(c);};

/**-----------------------------------
 *									*
 *			当前的运行环境			*
 *									*
 -----------------------------------*/
	//返回当前运行的欢迎
	PHP.$_ENV = function(){
		var env = window.navigator;
		return env;
	};

	//查看浏览器上的插件
	PHP.$_Plugin = function(){
		
	};

	//浏览器语言版本
	PHP.language = window.navigator.language;

	//运行的平台
	PHP.platform = window.navigator.platform;
	
	/**
	 * @func 浏览器运行的环境
	 * @return 返回一个数据
	 * ah:浏览者的屏幕高度
	 * aw:浏览者的屏幕宽度
	 * wh:屏幕的可用高度
	 * ww:屏幕的可用高度
	 * h:浏览器可用高度
	 * w:浏览器可用宽度
	 */
	PHP.screen = function(){
		var s = screen;
		var w = window;
		//console.log(s);
		//console.log(window.outerHeight);
		return [s.availHeight,s.availWidth,
			   w.outerHeight,w.outerWidth,
			   s.height,s.width];
	} 

	//cookie是否支持;
	PHP.is_cookie = window.navigator.cookieEnabled;
	
	//java是否支持;
	PHP.is_java = window.navigator.javaEnabled();




	/**
	 *	让PHP关键字成为全局函数
	 */
	window.PHP=PHP;
})();

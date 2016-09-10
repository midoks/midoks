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
				}else if(xhr.type=='JSON'){
					xhr.result(eval('('+xhr.XHR.responseText+')'));
				}else{
					xhr.result(xhr.XHR.responseXML);	
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
	}

	//因为ajax不能跨域操作的带,而想的解决方法
	/**
	 *	@param string url 网站的地址
	 *	@return string 返回的数据
	 */
	function create_iframe(url){
		var f;
		try{
			f = document.createElement('iframe');//非IE下
		}catch(e){
			f = document.createElement("<iframe></iframe>");//IE下 
		}
	    f.style.display='block';
		f.src=url;
		f.id='PHPjs';
		document.body.appendChild(f);
		console.log(document.getElementById('PHPjs'));
		return document.getElementById('PHPjs').document.body.innerText;
	}

   /**
	* file_get_contents 函数 获取的网页的数据
	* @param string url 网站的地址
	* @param mixed data 对返回的值,操作
	* @return string 获取网页的数据
	*/
	PHP.file_get_contents = function(url,data){
		/*if(typeof url ==='undefined'){
		
		}*/
		php_ajax('HTML',true).get(url,data);
		//create_iframe(url);
	}

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

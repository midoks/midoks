/**
 *	lzw 压缩数据算法
 */

/**
 *	@func lzw压缩/解压数据算法
 *	@param string s 压缩/解压/字符串
 *	@param boolean 0 压缩  |  1 解压 
 *	@reuturn binary string 返回压缩/解压二进制字符串
 */

function lzw(s, m){
	

	/**
	 *	@func  以unicode码编写
	 *	@param string str 要编码的字符串
	 *	@return 返回编码后的数据
	 */
	this.encode = function(str){//仅支持unicode码
		var result = '';
		for(var n=0; n<str.length; n++){
			var c = str.charCodeAt(n);
			if(c < 128){//ASCII码(二进制7内处理)
				result += String.fromCharCode(c);
			}else if(c>127 && c<2048){//(二进制7~12之间处理)
				result += String.fromCharCode( (c >> 6) | 192 );//192 (11000000)
				result += String.fromCharCode( (c & 63) | 128 );//128 (10000000)
			}else{//(二进制12~20之间处理)
				result += String.fromCharCode( (c>>12) | 224 ); //224 (11100000)
				result += String.fromCharCode( ((c >> 6) & 63) | 128);
				result += String.fromCharCode( (c & 63) | 128);
			}
		}
		return result;
	}

	/**
	 *	@func 解压
	 *	@param string str 要解压的字符串
	 *	@return string result 返回解压后的字符串
	 */
	this.decode = function(str){
		var result = '';
		var i = 0;
		var c1 = 0;
		var c2 = 0;
		var c3 = 0;
		while(i < str.length){
			c1 = str.charCodeAt(i);
			if(c1 < 128){////ASCII码(二进制7内处理)
				result += String.fromCharCode(c1);
				i++;
			}else if(c1 >191 && c1 < 224){//(二进制7~12之间处理)
				c2 = str.charCodeAt(i+1);
				result += String.fromCharCode(((c1 & 31) << 6) | (c2 & 63));
				i+=2;
			}else{//(二进制12~20之间处理)
				c2 = str.charCodeAt(i + 1);
             	c3 = str.charCodeAt(i + 2);
             	result += String.fromCharCode(((c1 & 15) << 12) | ((c2 & 63) << 6) | (c3 & 63));
				i += 3;
			}
		}
		return result;
	}


	/**
	 * @func lzw 压缩算法
	 * @param string str 要压缩的字符串
	 * @return string 返回压缩的字符串 
	 */
	this.compress = function(str){
		var str = this.encode(str);//转码
		var dic = new Array();//基础字典
		var chars = 256;//字符长度
		for(var i=0; i<chars; i++){
			dic[String(i)] = i;
		}
		//console.log(dic);
		var splited = new Array();//分割
		splited = str.split('');//分割字符串
		var buffer = new Array();//缓存数值
		var xstr = '';
		var result = new String('');//结果
		var size = splited.length;//长度
		for(var i=0; i<=size; i++){
			current = new String(splited[i]);
			xstr = (buffer.length == 0) ?
				   String(current.charCodeAt(0)) : (buffer.join('-')
				   + '-' + String(current.charCodeAt(0)));
			//console.log(xstr);
			if(dic[xstr] !== undefined){
				buffer.push(current.charCodeAt(0));
			}else{
				result += String.fromCharCode(dic[buffer.join('-')]);
				dic[xstr] = chars;
				chars++;
				buffer = new Array();
				buffer.push(current.charCodeAt(0));
			}
			//console.log('1' + result);
		}
		return result;		
	}

	/**
	 *	@func lzw解压
	 *	@param string str 要解压的字符串
	 *	@return string 返回解压后的字符串
	 */
	this.decompress = function(str){
		var dic = new Array();//基础字典
		var chars = 256;//字符长度
		for(var i=0; i<chars; i++){
			dic[i] = String.fromCharCode(i);
		}

		var original = new String(str);
		var splited = original.split('');//拆分
		var buffer = new String('');//缓存值
		var result = new String('');//结果值
		var chain = new String('');
		for(var i=0; i<splited.length; i++){
			var code = original.charCodeAt(i);
			var current = dic[code];
			//console.log(code);
			if(buffer == ''){
				buffer = current;
				result += current;
			}else{
				if(code <= 255){
					result += current;
					chain = buffer + current;
					dic[chars] = chain;
					chars++;
					buffer = current;
				}else{
					chain = dic[code];
					if(chain == null){
						chain = buffer + buffer.slice(0, 1);
					}
					result += chain;
					dic[chars] = buffer + chain.slice(0, 1);
					chars++;
					buffer = chain;
				}
			}
		}	
		result = this.decode(result);
		return result;
	}
	
	//压
	//var y = this.encode('你');
	//console.log(y);

	//解
	//var j = this.decode(y);
	//console.log(j);
	
	/*if(typeof m != Number){
		var method = 0;
	}else{
		var method = m;
	}*/
	


	if(m.toString()=='0'){//0 | 返回lzw压缩结果
		console.log(s);
		return this.compress(s);
	}else if(m.toString()=='1'){//1 | 返回lzw解压结果
		console.log(s);
		return this.decompress(s);
	}else{//参数不对
		return ;
	}
}

//--------------------------------------------------
//test
var t = lzw('你好', 0);
console.log('现在:'+ t + t.length);
//解压
var t = lzw(t, 1);
console.log('现在:'+ t + t.length);

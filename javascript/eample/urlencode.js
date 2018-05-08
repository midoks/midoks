/**
 *  @url http://www.blogjava.net/hadeslee/archive/2007/11/16/160544.html 参考地址
 *	@func urlencode 编码
 *	@param string url URL地址
 *	@return 返回编码后地址
 */
function urlencode(url){
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
function urldecode(url){
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

/*{ test : urlencode编码|解码 }*/
var url = urlencode('http://localhost/5/test.html');
console.log(url);
url = urldecode(url);
console.log(url);

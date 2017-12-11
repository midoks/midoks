/**
 *	@func base64编码
 *	@param string str 编码的字符串
 *	@param string 返回编码后的内容
 */
function base64_encode(str){
	var b64_encode = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
	//console.log(b64_encode.length);
	var out,i,len;
	var c1,c2,c3;

	len = str.length;
	i = 0;
	out = '';
	//console.log((c1 & 0x3) << 4);
	while(i<len){
		c1 = str.charCodeAt(i++) & 0xFF;//控制在8位内
		if(i==len){
			out += b64_encode.charAt(c1>>2);
			out += b64_encode.charAt((c1 & 0x3) << 4);
			out += '==';
			break;
		}

		c2 = str.charCodeAt(i++) & 0xFF;
		if(i==len){
			out += b64_encode.charAt(c1>>2);
			out += b64_encode.charAt(((c1 & 0x3) << 4) | ((c2 & 0xF0)>>4));
			out += b64_encode.charAt((c2 & 0xF)<<2);
			out += '=';
			break;
		}
		c3 = str.charCodeAt(i++) & 0xFF;
		out += b64_encode.charAt(c1>>2);
		out += b64_encode.charAt(((c1&0x3)<<4) | ((c2&0xF0)>>4));
		out += b64_encode.charAt(((c2&0xF)<<2) | ((c3&0xC0)>>6));
		out += b64_encode.charAt(c3&0x3F);		
	}
	return out;
}

/**
 *	@func base64解码
 *	@param string 要解码的内容
 *	@return string 返回解码后的内容
 */
function base64_decode(str){
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
	i = 0;
	len = str.length;
	out = '';
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

/*{test : 针对字母和数字 }*/
var e = base64_encode('123');
console.log(e);
var p = base64_decode(e);
console.log(p);

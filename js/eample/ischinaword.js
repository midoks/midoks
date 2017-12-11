/**
 *	@func 判断是否有汉子
 *	@param string str 要判断的字符串
 */
function is_cn(str) { 
	var reg=/[\u4E00-\u9FA5]/g 
	if (reg.test(str)){
		console.log("含有汉字");
	}else{
		console.log("不含有汉字");
	} 
}

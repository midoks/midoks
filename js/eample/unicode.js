/**
 *	@func 全部JS unicode的值
 *
 *	(实体)[&#+16进制] = unicode码
 */
function unicode(){
	var unicode = new Array();
	for(var i=0; i<0xFFFF; i++){
		unicode[i] = String.fromCharCode(i);
	}
	return unicode;
}

//展示
function show_unicode(){
	var j = unicode();
	var str = String('');
	for(var i=0; i<j.length; i++){
		str += i+':'+j[i] + "\t";
		if(i%20==0){
			str += "\r\n";
		}	
	}
	return str;
}

console.log(show_unicode());

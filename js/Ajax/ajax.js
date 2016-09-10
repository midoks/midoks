(function(){

	//获取xhr对象
	try{
		xhr = new XMLHttpRequest();//非IE中
	}catch(e){
		var v = ['Microsoft.XMLHTTP', 'MSXML.XMLHTTP', 'Microsoft.XMLHTTP', 'Msxml2.XMLHTTP.7.0', 'Msxml2.XMLHTTP.6.0', 'Msxml2.XMLHTTP.5.0', 'Msxml2.XMLHTTP.4.0', 'MSXML2.XMLHTTP.3.0', 'MSXML2.XMLHTTP'];//IE当中,各种测试
		for(var i;i<v.length;i++){
			try{
				xhr = new ActiveXObject(v[i]);
			}catch(e){
				continue;
			}
		}
	}
		
	xhr.onreadystatechange = function(){
		if(xhr.readyState==4 && xhr.status==200){
			alert(xhr.responseText);
			console.log('asd');
		}
	}

	

	//异步请求方法
	//xhr.open('GET','http://www.heibai.com',true);
	//xhr.send();
	//
	
	//POST请求方法
	xhr.open('POST','test.php',true);
	//设置HTTP头
	xhr.setRequestHeader('Content-type','application/x-www-form-urlencoded');
	xhr.send('F=Q&L=M');
})();

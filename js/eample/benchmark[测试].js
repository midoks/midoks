window.onload = function(){
/////////////////////////////////////////

/**
 *	@func 方法基准测试
 *	@param func 要测试的方法
 *	@param times 循环次数
 */
function benchmark(test,times){
	if(typeof times =='undefined'){
		times = 1;
	}
	var tbegin = (new Date()).getTime();
	var div = document.createElement('div');
	if(test){
		for(var i=0;i<times;i++){
			test();
		}
	}
	var tend = (new Date()).getTime();
	div.innerText = '测试结果为: 当前运行次数:'+ times + '运行总时间:' + (tend-tbegin) + 'ms' + '每次运行:' + (tend-tbegin)/times + 'ms';
	document.body.appendChild(div);
}
/*benchmark(function(){

	
	

},10);*/

//document.write(String.fromCharCode(72,69,76,76,100));
//lookascii();
/////////////////////////////////////////
}

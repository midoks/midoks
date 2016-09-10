var arr = [3, 5, 12, 55, 12, 321, 41, 0, 999];
/**
 * @func 数组转化为方法,并弹出值。
 */
function arr2func(){
	if( (arguments.length != 1) && !(arguments[0] instanceof Array) ){
		console.error("参数错误:方法 数组转方法");
		return;
	};

	for(var i = 0; i<arguments[0].length; i++){
		arguments[0][i] = (function(a){
			return function(){
				return a;
			};
		})(arguments[0][i]);
	}
	
	return arguments[0];
}

//test
arr = arr2func(arr);
//var t = (arr[0])();
console.log((arr[5])());

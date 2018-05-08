/*
	
   自己编写的JS库

*/

//方法1
(function(){
var PHP = {};
PHP.size = function(str){
	return str.length;
}
//window['PHP'] = PHP;
window.PHP = PHP;
})();


//方法2
(function(){
var PHP1 = {
	size:function(str){
		return this.length;
	}
};
window['PHP1'] = PHP1;
})();

//方法3
(function(){
	//申明一个自定义对象
	window['PHP3'] = {};
	function size(str){
		return str.length;
	}
	//window['PHP3']['size']=size;
})();
//方法4
(function(){
	function PHP4(){};
	PHP4.prototype.size = function(str){
		return str.length;
	};
	var PHP4 = new PHP4;
	window.PHP4 = PHP4;
})();



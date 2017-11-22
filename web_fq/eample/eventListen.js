/**
 *	比较通用型的事件监听
 *	兼容IE5.0+IE6.0和 火狐
 *	@param mixed elm  如 document.getElementById('body');
 *	@param string evType 事件的类型
 *	@param function fn 执行的方法
 *	@param boolean useCapture (一般为false)
 */
function eventListener(elm,evType,fn,useCapture){
	if(elm.addEventListener){
		elm.addEventListener(evType,fn,useCapture);
		return true;
	}else if(elm.attachEvent){
		var r = elm.attachEvent('on'+evType,fn);
	}else{
		elm['on'+evType] = fn;
	}
}

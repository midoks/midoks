/**
 *	@func JS解压缩
 *  @need 支持canvas
 */

/**
 *	@func 方法
 *	@param fn 文件
 *	@param callback 回调方法
 */
 
function LoadData(fn, callback){
	//创建对象
	var bCanvas = false;//是否可使用
	var oCanvas = document.createElement('canvas');
	if(oCanvas.getContext){//是否getContext这个功能
		var oCtx = oCanvas.getContext('2d');
		if(oCtx.getImageData){
			bCanvas = true;
		}
	}
	
	if(bCanvas){
		//添加数据图片
		var DataImg = new Image();
		DataImg.style.position = 'absolute';
		DataImg.style.left = '-10000px';
		//图片添加到body中
		document.body.appendChild(DataImg);
		DataImg.onload = function(){
			var iWidth = this.offsetWidth;//图片的宽度
			var iHeight = this.offsetHeight;//图片的高度
			oCanvas.width = iWidth;
			oCanvas.height = iHeight;
			oCanvas.style.width = iWidth + 'px';
			oCanvas.style.height = iHeight + 'px';
			//
			var  oText = document.getElementById('output');
			oCtx.drawImage(this, 0, 0);
			var oData = oCtx.getImageData(0, 0, iWidth, iHeight).data;
			var a = [];
			var len = oData.length;
			var p = -1;
			for(var i=0; i<len;i+=4){
				if(oData[i]>0){
					a[++p] = String.fromCharCode(oData[i]);
				}
			}
			var strData = a.join('');
			if(callback){
				callback(strData);
			}
			//删除标签
			document.body.removeChild(DataImg);
		}
		
		//加载的图片
		DataImg.src = fn;
	}
	//console.log(oCanvas.getContext('2d'));
}

LoadData('./test.png',function(data){
	console.log(data);
	eval(data);
	
	
	console.log(PHP.max(20, 10000));
});
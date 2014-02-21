/**
 * @func 通过flash获取网页中数据
 * author:midoks
 */

//声明对象
var GetUrlData = {
	
	
	MakeHtml:function(){
		var html = '';
		if(navigator.userAgent.match(/MSIN/)){
			var protocol = location.href.match(/^https/i) ? 'https://' : 'http://';
			//在IE浏览器插入
			html += '<object classid="clsid:d27cdb6e-ae6d-11cf-96b8-444553540000" codebase="'+protocol+'download.macromedia.com/pub/shockwave/cabs/flash/swflash.cab#version=9,0,0,0" width="200" height="200" id="movieId" align="middle"><param name="allowScriptAccess" value="always" /><param name="allowFullScreen" value="false" /><param name="movie" value="'+this.MoviePath+'" /><param name="loop" value="false" /><param name="menu" value="false" /><param name="quality" value="best" /><param name="bgcolor" value="#ffffff" /><param name="flashvars" value="id=123"/><param name="wmode" value="transparent"/></object>';
		}else{
			//在非IE中插入
			html += '<embed id="movieId" src="'+this.MoviePath+'" loop="false" menu="false" quality="best" bgcolor="#ffffff" width="200" height="200" name="movieId" align="middle" allowScriptAccess="always" allowFullScreen="false" type="application/x-shockwave-flash" pluginspage="http://www.macromedia.com/go/getflashplayer" flashvars="id=123" wmode="transparent" />';
		
		}
		return html;
	},
	
	test:function(arg){
		console.log(arg);
	},


};

package {
	//解决跨域问题,获取网页数据
	//Author:midoks
	
	//载入网络操作类
	import flash.net.*;
	//载入外部调用接口类
	import flash.external.ExternalInterface;
	//载入舞台
	import flash.display.Stage;
	import flash.display.StageScaleMode;//缩放模式类
	//载入Sprite
	import flash.display.Sprite;
	//加载安全设置类,可以再不同域内互相通信
	import flash.system.Security;
	//接受外部传进的值
	import flash.display.LoaderInfo;
	//载入时间调度
	import flash.utils.Timer;
	import flash.events.TimerEvent;
	
	//获取网页数据的类型
	public class GetUrlData extends Sprite{
		
		flash.system.Security.allowDomain("*");
		
		var url:String = '';
		
		//构造方法
		/**
		 *	实例URLLoader对象,设置设置监听时间
		 */
		public function GetUrlData() {
			//stage.scaleMode = StageScaleMode.EXACT_FIT;
			flash.system.Security.allowDomain("*");
			
			//外部调用[也就是JS调用]
			ExternalInterface.addCallback("test2",test2);
		}
	
		//测试，返回test
		public function test2(htmljs:String){
			//url = htmljs; 
			ExternalInterface.call('GetUrlData.test',htmljs);
		}
	
		
	}//类结束
}//包结束
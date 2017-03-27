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
	
	//获取网页数据的类型
	public class GetUrlData extends Sprite{
		
		
		//构造方法
		/**
		 *	实例URLLoader对象,设置设置监听时间
		 */
		public function GetUrlData() {
			
			//把值传给JS.
			//会直接触动JS代码运行
			ExternalInterface.call('GetUrlData.test','123123');
		}
		
		
		
		
		
		
	}//类结束
}//包结束
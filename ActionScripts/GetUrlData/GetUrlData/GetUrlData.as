package {
	//解决跨域问题,获取网页数据
	//Author:midoks  
	//版本 version 1.0
	
	//载入网络操作类
	import flash.net.*;
	//载入外部调用接口类
	import flash.external.ExternalInterface;
	//载入舞台
	import flash.display.Stage;
	import flash.display.StageScaleMode;//缩放模式类
	import flash.display.*;
		
	//载入Sprite
	import flash.display.Sprite;
	//加载安全设置类,可以再不同域内互相通信
	import flash.system.Security;
	//接受外部传进的值
	import flash.display.LoaderInfo;
	//载入时间操作
	import flash.events.*;
	//载入系统设置
	import flash.system.*;
	
	//获取网页数据的类型
	public class GetUrlData extends Sprite{
		
		//定义变量,外面接受的ID值
		private var id:String = '';
		private var loader:URLLoader;//URLLoader 类
		//private var GetUrlDataSign:Boolean = false;//是否获取数据 默认true;		
		
		//构造方法
		/**
		 *	实例URLLoader对象,设置设置监听时间
		 */
		public function GetUrlData() {
			//stage.scaleMode = StageScaleMode.EXACT_FIT;
			flash.system.Security.allowDomain("*");//安全设置
			//返回编码为 UTF-8 或系统代码页的输入字符串的转义副本，具体取决于 System.useCodePage 的值。
			//可现实中文
			System.useCodePage = true;
			
			loader = new URLLoader();//URLLoader 类
			loader.dataFormat = URLLoaderDataFormat.TEXT;//返回类型
			WebListener(loader);//监听事件
			
			//传输准备完毕的信号
			ExternalInterface.call('G.FlashTrigger',true);
			
			//外部调用[也就是JS调用] @func 获取数据
			ExternalInterface.addCallback('get',get);
			//外部调用[也就是JS调用]
			//ExternalInterface.addCallback('test',test);
	
		}
		
		public function get(url:String){
			//传来了网址必须要有https:// | http://
			/*var reg:RegExp = /(https:|http:)\/\//;
			var e = reg.test(url);
			if(e){*/
				//获取网页中数据
				var request:URLRequest = new URLRequest(url);//请求的地址
				try {
                	request.method = URLRequestMethod.GET;//请求方式
					loader.load(request);
					
            	} catch (error:Error) {
                	ExternalInterface.call('G.error','无法加载所请求的文档!');
            	}
			/*}else{
				ExternalInterface.call('G.test',e);
				ExternalInterface.call('G.error','域名不符合要求!');
			}
			ExternalInterface.call('G.test',e);*/
		}
		
		//测试，返回test
		public function test(){
			var str:String = '123';
			return str;
		}
		
		//获取网页内容监听
		private function WebListener(dispatcher:IEventDispatcher):void{
			dispatcher.addEventListener(Event.COMPLETE, completeHandler);//数据加载完成事件
            dispatcher.addEventListener(Event.OPEN, openHandler);//资源打开事件
            dispatcher.addEventListener(ProgressEvent.PROGRESS, progressHandler);//资源进程事件
            dispatcher.addEventListener(SecurityErrorEvent.SECURITY_ERROR, securityErrorHandler);//安全错误资源
            dispatcher.addEventListener(HTTPStatusEvent.HTTP_STATUS, httpStatusHandler);//http状态事件
            dispatcher.addEventListener(IOErrorEvent.IO_ERROR, ioErrorHandler);//IO错误状态事件
		}
		
		//数据加载完成事件
		private function completeHandler(event:Event):void{
			var loader:URLLoader = URLLoader(event.target);
            //trace("完成后的数据： " + loader.data);
			ExternalInterface.call('G.result',loader.data);
		}
		
		//资源打开事件
		private function openHandler(event:Event):void{
			ExternalInterface.call('G.test','资源打开');
			ExternalInterface.call('G.test',"openHandler: " + event);
		}
		
		//加载过程调度事件
		private function progressHandler(event:ProgressEvent):void{
			ExternalInterface.call('G.test','加载进程');
			ExternalInterface.call('G.progress',"progressHandler loaded:" + event.bytesLoaded + " total: " + event.bytesTotal);
			
		}
		
		//安全错误资源
		private function securityErrorHandler(event:SecurityErrorEvent):void{
			
			ExternalInterface.call('G.result','false');
			ExternalInterface.call('G.test','不安全访问');
			
		}
		
		//http状态事件
		private function httpStatusHandler(event:HTTPStatusEvent):void{
			
		}
		
		//IO错误状态事件
		private function ioErrorHandler(event:IOErrorEvent):void{
			
		}
		
		
		
		
		
		
		
		
	}//类结束
}//包结束
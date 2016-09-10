package{
	//解决跨域问题,获取网页数据
	//Author:midoks
	
	import flash.display.Stage;
	import flash.display.Sprite;
	import flash.display.LoaderInfo;
	import flash.display.StageScaleMode;
	import flash.events.*;
	import flash.display.StageAlign;
	import flash.display.StageScaleMode;
	import flash.external.ExternalInterface;
	import flash.system.Security;
	import flash.utils.*;
	import flash.system.System;
	
	public class copyClipboard extends Sprite{
		
		public var str:String = '';
		private var button:Sprite;
		
		public function copyClipboard(){
			//stage.scaleMode = StageScaleMode.EXACT_FIT;
			flash.system.Security.allowDomain("*");
			flash.system.Security.allowInsecureDomain("*");
			
			
			button = new Sprite();
			button.buttonMode = true;
			button.useHandCursor = true;
			button.graphics.beginFill(0xCCFF00);
			button.alpha = 0.0;
			addChild(button);
			button.addEventListener(MouseEvent.CLICK, copy);
			
			//传输准备完毕的信号
			ExternalInterface.call('COPY.FlashTrigger', true);
			//测试
			//ExternalInterface.call('COPY.test', "Nihao");
			
			//外部JS可调用方法
			ExternalInterface.addCallback("set", set);
		}
		
		public function set(clipText:String){
			//ExternalInterface.call('COPY.test', clipText);//复制完成
			str = clipText;
			//ExternalInterface.call('COPY.test', str);//复制完成
			//ExternalInterface.call('COPY.test', System.setClipboard('totalMemory'));//复制完成
			//ExternalInterface.call('COPY.compele', true);
			//System.setClipboard("System.totalMemory: " + System.totalMemory);
			mouse_click();
			//ExternalInterface.call('COPY.Compele', true);
		}
		
		//模拟数据点击
		public function mouse_click(){
			var mouseEvent:MouseEvent = new MouseEvent(MouseEvent.CLICK);
			mouseEvent.buttonDown = true;
			button.dispatchEvent( mouseEvent );
			ExternalInterface.call('COPY.test', '模拟点击事件');
		}
		
		public function copy(event:Event):void{
			ExternalInterface.call('COPY.test', '已复制到剪贴板上');//复制完成
			ExternalInterface.call('COPY.Compele', true);
			System.setClipboard( str );
		}
		
		public function copy2(str:String):void{
			System.setClipboard( str );
		}
		
		
		
	}//类结束
}//包结束
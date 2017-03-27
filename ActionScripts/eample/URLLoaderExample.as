package {
    import flash.display.Sprite;
    import flash.events.*;
    import flash.net.*;

    public class URLLoaderExample extends Sprite {
        public function URLLoaderExample(str:String) {
            var loader:URLLoader = new URLLoader();
            configureListeners(loader);

            var request:URLRequest = new URLRequest(str);
            try {
                loader.load(request);
            } catch (error:Error) {
                trace("Unable to load requested document.");
            }
        }

        private function configureListeners(dispatcher:IEventDispatcher):void {
            dispatcher.addEventListener(Event.COMPLETE, completeHandler);
            dispatcher.addEventListener(Event.OPEN, openHandler);
            dispatcher.addEventListener(ProgressEvent.PROGRESS, progressHandler);
            dispatcher.addEventListener(SecurityErrorEvent.SECURITY_ERROR, securityErrorHandler);
            dispatcher.addEventListener(HTTPStatusEvent.HTTP_STATUS, httpStatusHandler);
            dispatcher.addEventListener(IOErrorEvent.IO_ERROR, ioErrorHandler);
        }

        private function completeHandler(event:Event):void {
            var loader:URLLoader = URLLoader(event.target);
            trace("完成后的数据： " + loader.data);
    
            //var vars:URLVariables = new URLVariables(loader.data);
            //trace("The answer is " + vars.answer);
        }

        private function openHandler(event:Event):void {
            trace("打开资源: " + event);
        }

        private function progressHandler(event:ProgressEvent):void {
            trace("进程响应字节 下载：" + event.bytesLoaded + " 总共字节: " + event.bytesTotal);
        }

        private function securityErrorHandler(event:SecurityErrorEvent):void {
            trace('安全错误数据: ' + event);
        }

        private function httpStatusHandler(event:HTTPStatusEvent):void {
            trace("网站响应状态" + event);
        }

        private function ioErrorHandler(event:IOErrorEvent):void {
            trace("ioErrorHandler: " + event);
        }
    }
}
<?php
/**
 *	@func midoks 进行了部分改写,才能与我的WP微信机器人结合在一起
 *	@email midoks@163.com
 *	@blog midoks.cachecha.com
 *	@source www.weiphp.cn
 *	@dir Addons/Extensions/Model/WeixinAddonModel.class.php
 */

namespace Addons\Extensions\Model;

use Home\Model\WeixinModel;

/**
 * Extensions的微信模型
 */
class WeixinAddonModel extends WeixinModel {
	function reply($dataArr, $keywordArr = array()) {
		$map ['id'] = $keywordArr ['aim_id'];
		$info = M ( 'extensions' )->where ( $map )->find ();
		
		$post_data = $this->get_res_data();
		$url = $this->get_url($info['api_url']);
		$response = $this->deliver($url, $post_data);
		echo ($response);
	}

	public function default_url($purl){
		$url = $purl.
			'&signature=786f054ada120e459426757946d5c24d36adc1d7&timestamp=1405333563&nonce=193938084';
		return $url;
	}


	public function get_url($purl){
		$url = $purl;
		$url .= '&web=true';

		$sign = false;
		if(isset($_GET['signature'])){
			$url .= '&signature='.$_GET['signature'];
			$sign = true;
		}

		if(isset($_GET['echostr'])){
			$url .= '&echostr='.$_GET['echostr'];
		}

		if(isset($_GET['timestamp'])){
			$url .= '&timestamp='.$_GET['timestamp'];
			$sign = true;
		}

		if(isset($_GET['nonce'])){
			$url .= '&nonce='.$_GET['nonce'];
			$sign = true;
		}

		if(!$sign){
			return $this->default_url();
		}
		return $url;
	}

	//请注意这个函数!!!
	public function get_res_data(){
		if($data = file_get_contents('php://input')){
			return $data;
		}elseif(isset($GLOBALS['HTTP_RAW_POST_DATA'])){
			return $GLOBALS['HTTP_RAW_POST_DATA'];//POST数据
		}else{

$name = isset($_GET['kw'])? $_GET['kw']:'测试';
$ttt = <<<EOT
<xml><ToUserName><![CDATA[gh_aa1df1b1f411]]></ToUserName>
<FromUserName><![CDATA[oTGh6jlvP2S56YW2-7GJSl8UCo_0]]></FromUserName>
<CreateTime>1405322624</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[$name]]></Content>
<MsgId>6035814710609440344</MsgId>
</xml>
EOT;
			return $ttt;
		}
	}

	public function deliver($url, $data){
		$go = curl_init();
		curl_setopt($go, CURLOPT_URL, $url);
		curl_setopt($go, CURLOPT_FOLLOWLOCATION, 1);
		curl_setopt($go, CURLOPT_MAXREDIRS, 5);
		curl_setopt($go, CURLOPT_HEADER, 0);
		curl_setopt($go, CURLOPT_RETURNTRANSFER, 1);
		curl_setopt($go, CURLOPT_TIMEOUT, 10);
		if(!empty($data)){//POST Data
			curl_setopt($go, CURLOPT_POST, 1);
			curl_setopt($go, CURLOPT_POSTFIELDS ,$data);
		}
		$response = curl_exec($go);
		curl_close($go);
		return $response;
	}

	
	// 关注公众号事件
	public function subscribe() {
		return true;
	}
	
	// 取消关注公众号事件
	public function unsubscribe() {
		return true;
	}
	
	// 扫描带参数二维码事件
	public function scan() {
		return true;
	}
	
	// 上报地理位置事件
	public function location() {
		return true;
	}
	
	// 自定义菜单事件
	public function click() {
		return true;
	}
}
        	

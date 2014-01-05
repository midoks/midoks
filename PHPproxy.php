<?php
/**
 *	@func 自从我的BLOG移到SAE上,而我的域名没有备案,百度直接绑定时不行的了.
 *	我突然就想到了翻墙-代理等等东西,我就想没有备案的域名的情况下
 *	也能享受更好的服务而已.
 */

//实际地址
define('DOMAIN', '---');
//现有使用地址
define('NOW_DOMAIN', '----');
class proxy{

	//常见header文件匹配
	public function mimeTypeMatch($url){
		$allow = array(
			'css' => 'text/css',
			'js'  => 'application/x-javascript', 
			'png' => 'image/png',
			'ico' => 'image/x-icon',
			'jpg' => 'image/jpeg',
			'gif' => 'image/gif',
			'swf' => 'application/x-shockwave-flash',
		   	'htm' => 'text/html', 
			'html' => 'text/html',	
		);
		foreach($allow as $k=>$v){
			if(preg_match("/.{$k}/", $url)){
				//var_dump($v);
				return $allow[$k];
			}
		}
		return false;
	}

	public function _get(){
		$ch = curl_init();

		//设置URL地址
		if(isset($_SERVER['REQUEST_URI'])){
			//echo DOMAIN.$_SERVER['REQUEST_URI'];
			curl_setopt($ch, CURLOPT_URL , DOMAIN.$_SERVER['REQUEST_URI']);
		}else{
			curl_setopt($ch, CURLOPT_URL , DOMAIN);
		}

		curl_setopt($ch, CURLOPT_FOLLOWLOCATION , 1);//设置是否可以跳转
		curl_setopt($ch, CURLOPT_MAXREDIRS , 5);//设置跳转的次数
		curl_setopt($ch, CURLOPT_HEADER , 0);//头文件
		curl_setopt($ch, CURLOPT_RETURNTRANSFER , 1);//返回数据流

		//referer
		if(isset($_SERVER['HTTP_REFERER'])){
			curl_setopt($ch, CURLOPT_REFERER , $_SERVER['HTTP_REFERER']);
		}

		//user-agent
		if(isset($_SERVER['HTTP_USER_AGENT'])){
			curl_setopt($ch, CURLOPT_USERAGENT , $_SERVER['HTTP_USER_AGENT']);
		}
		//COOKIE
		if(isset($_SERVER['HTTP_COOKIE'])){
			//var_dump($_SERVER['HTTP_COOKIE']);
			curl_setopt($ch, CURLOPT_COOKIE , $_SERVER['HTTP_COOKIE']);
		}
		//POST数据 && $GLOBALS['HTTP_RAW_POST_DATA']
		if((isset($_POST) && !empty($_POST)) || (isset($GLOBALS['HTTP_RAW_POST_DATA']))){
			curl_setopt($ch, CURLOPT_POST ,1);
			if(isset($GLOBALS['HTTP_RAW_POST_DATA'])){
				$raw_Data = $GLOBALS['HTTP_RAW_POST_DATA'];
			}else{
				$raw_Data = file_get_contents('php://input');
			}
				
			if(!empty($raw_Data)){
				curl_setopt($ch, CURLOPT_POSTFIELDS , $raw_Data);
			}else{
				curl_setopt($ch, CURLOPT_POSTFIELDS , http_build_query($_POST));
			}
			
		}

		$data = curl_exec($ch);
		curl_close($ch);
		
		if($_SERVER['REQUEST_URI']){
			$h = $this->mimeTypeMatch($_SERVER['REQUEST_URI']);
			//var_dump($h);exit;
			if($h){
				header("Content-type: ".$h);
			}
		}

		if(!empty($data) && false!=$data){
			if('/wp-login.php'==$_SERVER['REQUEST_URI']){
				
			}else if('/wp-admin'==substr($_SERVER['REQUEST_URI'], 0, 9)){
			
			}else{
				$data = str_replace(DOMAIN, NOW_DOMAIN, $data);
			}
			echo($data);
		
		}
	}

	public function run(){
		$this->_get();
	}
}
$obj = new proxy();
$obj->run();
?>

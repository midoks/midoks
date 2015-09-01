<?php
/**
 *	@func 自从我的BLOG移到SAE上,而我的域名没有备案,百度直接绑定时不行的了.
 *	我突然就想到了翻墙-代理等等东西,我就想没有备案的域名的情况下
 *	也能享受更好的服务而已.
 */

//针对aliapp的云,不能把所有资源统一定位到index.php上。
//所有要做如下的处理

//1.图片url,不替换
//2.css资源.不替换
//3.站外链接,不替换
//4.文章链接不替换

//实际地址
define('DOMAIN', 'http://midoks.duapp.com');
//现有使用地址


if($_SERVER['HTTP_HOST'] == 'midoks.aliapp.com'){
	define('NOW_DOMAIN', 'http://midoks.aliapp.com');
}else{
	//define('NOW_DOMAIN', 'http://midoks.aliapp.com');
	define('NOW_DOMAIN', 'http://t2.cn');
}




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

		//curl_setopt($ch, CURLOPT_FOLLOWLOCATION , 1);//设置是否可以跳转
		$this->curl_redir_exec($ch);
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
		$data_error = curl_errno($ch);
		curl_close($ch);

		//var_dump($data);
		//var_dump($data_error);
		
		if($_SERVER['REQUEST_URI']){
			$h = $this->mimeTypeMatch($_SERVER['REQUEST_URI']);
			if($h){
				header("Content-type: ".$h);
			}
		}

		if(!empty($data) && false!=$data){
			if('/wp-login.php'==$_SERVER['REQUEST_URI']){
				
			}else if('/wp-admin'==substr($_SERVER['REQUEST_URI'], 0, 9)){
			
			}else{
				//获取需要替换的url
				$data = $this->getLinksAndReplace($data);
				//exit();
				//$data = str_replace(DOMAIN, NOW_DOMAIN, $data);
			}
            $server_name = "<!-- aliapp proxy -->";
		    $data = preg_replace("/<head>(.*)<\/head>/ims", "\\0 \n".$server_name, $data);
		    echo($data);
		}
	}


	//匹配所有的URL的链接
	public function getLinksAndReplace($data){
		$len = strlen(DOMAIN);

		//匹配所有a链接
		preg_match_all("~<a href=[\'\"]?([^\'\"]+).*?>~", $data, $alinks);
		//var_dump($alinks);

		foreach ($alinks[1] as $key => $value) {
			if(substr($value, 0, $len) == DOMAIN){
				$revalue = str_replace(DOMAIN, NOW_DOMAIN, $value);
				//var_dump($revalue);

				//var_dump($value);

				if($this->isNeedReplace($value)){
					$data = str_replace($value, $revalue, $data);
				}


				
			}
		}

		//配置所有link链接
		preg_match_all('/<link.+?href=(\'|")(.+?)\\1/s', $data, $llinks);
		//var_dump($llinks);

		preg_match_all("~<script.+?src=(\'|\")(.+?)\\1~i", $data, $slinks);

		//preg_match_all("~[\'\"]http[s]?\:\/\/(.*)[\'\"][^<>]~i", $data, $alllinks);
		//var_dump($slinks);
		return $data;
	}

	private function isNeedReplace($url){
		$urlStruct = pathinfo($url);

		if(isset($urlStruct['extension'])  && $urlStruct['extension'] == 'html' ){
			return false;
		}
		//var_dump($url);
		return true;
	}

	//过滤文件
	private function filter_file($list){
		foreach($list as $k=>$v){
			if(preg_match("/.{$k}/", $url)){
				//var_dump($v);
				return $allow[$k];
			}
		}

	}

	private function curl_redir_exec($ch, $debug=""){
		static $curl_loops = 0;
		static $curl_max_loops = 20;
		
		if ($curl_loops++ >= $curl_max_loops){
			$curl_loops = 0;
			return FALSE;
		}
		curl_setopt($ch, CURLOPT_HEADER, true);
		curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

		$data = curl_exec($ch);
		$data = $data . "123";
		$debbbb = $data;

		//var_dump($data);
		//exit;
		
		list($header) = explode("\n\n", $data, 2);

		//var_dump($header, $data);

		$http_code = curl_getinfo($ch, CURLINFO_HTTP_CODE);
		
		if ($http_code == 301 || $http_code == 302) {
			$matches = array();
			preg_match('/Location: (.*?)\n/', $header, $matches);
			$url = @parse_url(trim(array_pop($matches)));
			//print_r($url);
			
			if (!$url){
				//couldn't process the url to redirect to
				$curl_loops = 0;
				return $data;
			}
			
			$last_url = parse_url(curl_getinfo($ch, CURLINFO_EFFECTIVE_URL));
			/*    if (!$url['scheme'])
			$url['scheme'] = $last_url['scheme'];
			if (!$url['host'])
			$url['host'] = $last_url['host'];
			if (!$url['path'])
			$url['path'] = $last_url['path'];*/

			//var_dump($url);
			/*if(isset($url['query'])){
				$url_query = $url['query'];
			}else{
				$url_query = "";
			}*/

			$new_url = $url['scheme'] . '://' . $url['host'] . $url['path'] . (isset($url['query']) ? '?'.$url['query']:'');
			curl_setopt($ch, CURLOPT_URL, $new_url);
			//    debug('Redirecting to', $new_url);
			return $this->curl_redir_exec($ch);
		} else {
			$curl_loops=0;
			return $debbbb;
		}
	}

	public function run(){
		$this->_get();
	}
}
$obj = new proxy();
$obj->run();
//echo 'hh';
?>

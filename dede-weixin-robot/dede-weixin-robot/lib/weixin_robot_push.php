<?php
/**
 *	@func 微信 数据推送
 */
class weixin_robot_push{

	public function __construct($ai, $as){
		$this->ai = $ai;
		$this->as = $as;
	}

   /**
 	* @func 获取远程数据
	* @param string $url 网络地址
	* @param string $json json数据格式
	* @ret string 返回响应数据
 	*/
	public function get($url, $json = ''){
		//初始化连接
		$go = curl_init();
		//设置URL地址
		curl_setopt($go, CURLOPT_URL , $url);
		curl_setopt($go, CURLOPT_HEADER , 0);
		//设置是否可以跳转
		curl_setopt($go, CURLOPT_FOLLOWLOCATION , 1);
		//设置跳转的次数
		curl_setopt($go, CURLOPT_MAXREDIRS , 30);
		//curl_setopt($go, CURLOPT_USERGENT , $_SERVER['HTTP_USER_AGENT']);
		//头文件
		curl_setopt($go, CURLOPT_HEADER , 0);
		//返回数据流
		curl_setopt($go, CURLOPT_RETURNTRANSFER , 1);
		//支持HTTPS
		curl_setopt($go, CURLOPT_SSL_VERIFYPEER, false);
		//curl_setopt($go, CURLOPT_SSL_VERIFYHOST, false);
	
		//POST数据
		//curl_setopt($go, CURLOPT_POST ,1);
		//curl_setopt($go, CURLOPT_POSTFIELDS ,$args);
		if(!empty($json)){
			curl_setopt($go, CURLOPT_POST ,1);
			curl_setopt($go, CURLOPT_POSTFIELDS ,$json);
		}
		$data = curl_exec($go);
		curl_close($go);
		return $data;
	}

	/**
	 *	@func 获取token (再优化处理)
	 *	@param string $ai
	 *	@param $string $as
	 *	@return array array('access_token'=>'*','expires_in'=>'')
	 */
	public function Token(){
		$tk = get_option('weixin_robot_token');
		if('{'==substr($tk, 0, 1)){
			$tk = json_decode($tk, true);
		}
		//echo '<pre>';
		//var_dump($tk);

		//var_dump('as过期:'.date('Y-m-d h:i:s',$tk['expires_in']));
		//var_dump('当前:'. date('Y-m-d h:i:s',time()));
		if((isset($tk['access_token'])) && isset($tk['expires_in']) && ($tk['expires_in']>time()) ){
			//$tk = json_decode($tk, true);
			return $tk;
		}
///////////////////////////////////////////

		$ai = $this->ai;
		$as = $this->as;
		$url = "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={$ai}&secret={$as}";
		$data = $this->get($url);
		if(!$data){
			return false;
		}
	
////////////////////////////////////////////
		$data = json_decode($data, true);
		$data['expires_in'] = time()+5400;
		//var_dump($data);
		update_option('weixin_robot_token', json_encode($data));
		return $data;
	}


	/**
	 *	@func 推送文本信息(通过测试)
	 *	@param string $UserID 用户ID
	 *	@param string $Text 文本信息
	 *	@json
	 */
	public function toText($UserID, $Text){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"text\",
    		\"text\":
    		{
         		\"content\":\"{$Text}\"
   			 }
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}

	/**
	 *	@func 推送图片信息
	 *	@param string $UserID 用户ID
	 *	@param string $media_id 图片ID
	 *	@ret json
	 */
	public function toPic($UserID, $media_id){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"image\",
    		\"image\":
    		{
         		\"media_id\":\"{$media_id}\"
   			 }
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}

	/**
	 *	@func 推送语音消息
	 *	@param string $UserID 用户ID
	 *	@param string $media_id 语音ID
	 *	@ret json 
	 */
	public function toVoice($UserID, $media_id){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"voice\",
    		\"voice\":
    		{
         		\"media_id\":\"{$media_id}\"
   			 }
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}

	/**
	 *	@func 推送视频消息
	 *	@param string $UserID 用户ID
	 *	@param string $media_id 语音ID
	 *	@param string $thumb_media_id 视频缩略图的媒体ID
	 *	@ret json
	 */
	public function toVideo($UserID, $media_id, $thumb_media_id){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"video\",
    		\"video\":
    		{
				\"media_id\":\"{$media_id}\",
				\"thumb_media_id\":\"{$thumb_media_id}\"
   			 }
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}
	/**
	 *	@func 推送音乐消息
	 *	@param string $UserID 用户ID
	 *	@param string $title 音乐标题
	 *	@param string $description 音乐描述
	 *	@param string $musicurl 音乐链接
	 *	@param string $hqmusicurl 高品质音乐链接，wifi环境优先使用该链接播放音乐
	 *	@param string $thumb_media_id 缩略图的媒体ID
	 *	@ret json
	 */
	public function toMusic($UserID, $title, $description, $musicurl, $hqmusicurl, $thumb_media_id){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"music\",
    		\"music\":
    		{
				\"title\":\"{$title}\",
      			\"description\":\"{$description}\",
      			\"musicurl\":\"{$musicurl}\",
      			\"hqmusicurl\":\"{$hqmusicurl}\",
      			\"thumb_media_id\":\"{$thumb_media_id}\"
   			 }
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}

	/**
	 *	@func 推送图文消息(通过测试)
	 *	@param string $UserID 用户ID
	 *	@param array(
	 *		array('title'=>'', 'description'=>'', 'url==>'', 'picurl'=>'')
	 *	)
	 *	@ret json
	 */
	public function toPicText($UserID, $info){
		$data = $this->Token();
		if(!$data){
			return false;
		}

		if(empty($info)){
			return false;
		}

		$str = '[';
		foreach($info as $k=>$v){
			$str .= '{';
			$str .= '"title":"'.$v['title'].'",';
			$str .= '"description":"'.$v['description'].'",';
			$str .= '"url":"'.$v['url'].'",';
			$str .= '"picurl":"'.$v['picurl'].'"';
			$str .= '},';
		}
		$str .= ']';

		$tk = $data['access_token'];
		$url ="https://api.weixin.qq.com/cgi-bin/message/custom/send?access_token={$tk}";
		$json = "{
    		\"touser\":\"{$UserID}\",
    		\"msgtype\":\"news\",
			\"news\":{
        		\"articles\":{$str};
            }  	
		}";
		$data = $this->get($url, $json);
		if(!$data){
			return false;
		}
		return $data;
	}
	
}
?>

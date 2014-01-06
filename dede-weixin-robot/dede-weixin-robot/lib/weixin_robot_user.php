<?php
/**
 *	微信用户管理
 */
class weixin_robot_user{

	public $ai = '';
	public $as = '';

	/**
	 * @func 构造函数并初始化
	 */
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
	public function get($url, $file = ''){
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
		if(!empty($file)){
			curl_setopt($go, CURLOPT_POST ,1);
			curl_setopt($go, CURLOPT_POSTFIELDS ,$file);
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
	 *	@func 获取用户信息
	 *	@param string 普通用户的表示,对当前公众号唯一
	 *	@return array
	 */
	public function getUserInfo($openID){
		$token = $this->Token();
		$token = $token['access_token'];
	
		$url = "https://api.weixin.qq.com/cgi-bin/user/info?access_token={$token}&openid={$openID}";
		$uinfo = $this->get($url);
		if($uinfo){
			return json_decode($uinfo, true);
		}
		return false;
	}

	/**
	 * @func			获取关注者列表
	 * @param 	string 	下一个列表openid
	 * @ret array
	 */
	public function getUserList($next_openid=''){
		$token = $this->Token();
		$token = $token['access_token'];

		$url = "https://api.weixin.qq.com/cgi-bin/user/get?access_token={$token}&next_openid={$next_openid}";
		if($ulist = $this->get($url)){
			return $ulist;
		}
		return false;
	}


}
?>

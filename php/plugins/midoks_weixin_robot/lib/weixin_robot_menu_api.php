<?php
/**
 *	@func 微信自定义菜单功能
 *	@author midoks
 *	@link midoks.cachecha.com
 */
class weixin_robot_menu_api{

	public $ai = '';
	public $as = '';
	

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
	 *	@func 获取token (再再优化处理)
	 *	@param string $ai
	 *	@param $string $as
	 *	@return array array('access_token'=>'*','expires_in'=>'')
	 */
	public function Token_1(){
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

	public function Token_2(){

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

	public function Token(){
		$data = $this->Token_1();
		if(isset($data['access_token'])){
			return $data;
		}
		return false;
	}

	//获取菜单
	public function menuGet(){
		$data = $this->Token();
		
		if(!$data){
			return false;
		}


		$token = $data['access_token'];
		$url = "https://api.weixin.qq.com/cgi-bin/menu/get?access_token={$token}";
		$data = $this->get($url);

			
		$_data = json_decode($data, true);
		if(isset($_data['errcode'])){
			$this->Token_2();
			return $data;	
		}
		return $data;
	}

	//设置当前菜单
	public function menuSet($json){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$token = $data['access_token'];
		$url = "https://api.weixin.qq.com/cgi-bin/menu/create?access_token={$token}";
		$data = $this->get($url, $json);
		//var_dump($data);
		$_data = $data;
		$_data = json_decode($_data, true);
		if(isset($_data['errcode'])){
			$this->Token_2();
			return $data;	
		}
		return $data;
	}

	//删除当前菜单
	public function menuDel(){
		$data = $this->Token();
		if(!$data){
			return false;
		}
		$token = $data['access_token'];
		$url = "https://api.weixin.qq.com/cgi-bin/menu/delete?access_token={$token}";
		$data = $this->get($url);

		$_data = json_decode($data, true);
		if(isset($_data['errcode'])){
			$this->Token_2();
			return $data;	
		}
		return $data;
	}
}
?>

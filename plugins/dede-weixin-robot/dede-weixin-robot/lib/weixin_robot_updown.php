<?php
/**
 * @func 微信接口上传下载接口文件	
 */
include(dirname(__FILE__).'/mimetypes.class.php');
class weixin_robot_updown{

	public $token = null;

	/**
	 * @func 构造函数并初始化
	 */
	public function __construct($token){
		$this->token = $token;
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
	 * @func 上传文件数组
	 * @param array $array 上传文件数组
	 * @param string $type 上传类型
	 */
	private function up($array,$type){
		$token = $this->token;
		$url = "http://file.api.weixin.qq.com/cgi-bin/media/upload?access_token={$token}&type={$type}";
		return $this->get($url, $array);
	}

	/**
	 *	@func 图片上传
	 *	@param string $imgAddr 图片地址(绝对路径)
	 *	@return id
	 */
	public function ImageUp($ImgAddr){
		if(file_exists($ImgAddr) && (filesize($VoiceAddr) <= (128*1024))){
			$arr = array('media' => '@'.$ImgAddr,);
			$data = $this->up($arr, 'image');
			$data = json_decode($data, true);
			if(isset($data['errcode'])){
				return false;
			}else{
				return $data['media_id'];
			}
		}
		return false;
	}

	/**
	 *	@func 语音上传
	 *	@param string $VoiceAddr (绝对地址)
	 *	@return id
	 */
	public function VoiceUp($VoiceAddr){
		if(file_exists($VoiceAddr) && (filesize($VoiceAddr) <= 256*1024)){
			$arr = array(
				'media' => '@'.$VoiceAddr,
			);
			$data = $this->up($arr, 'voice');
			$data = json_decode($data, true);
			if(isset($data['errcode'])){
				return false;
			}else{
				return $data['media_id'];
			}
		}
		return false;
	}

	/**
	 * @func 视频上传
	 * @param string $VideoAddr 视频地址(绝对路径)
	 * @ret id
	 */
	public function VideoUp($VideoAddr){
		if(file_exists($VideoAddr) && (filesize($VideoAddr) <= 1024*1024)){
			$arr = array(
				'media' => '@'.$VideoAddr,
			);
			$data = $this->up($arr, 'voice');
			$data = json_decode($data, true);
			if(isset($data['errcode'])){
				return false;
			}else{
				return $data['media_id'];
			}
		}
		return false;
	}

	/**
	 * @func 视频上传
	 * @param string $VideoAddr 视频地址(绝对路径)
	 * @ret id
	 */
	public function ThumbUp($ThumbAddr){
		if(file_exists($ThumbAddr) && (filesize($ThumbAddr) <= 64*1024)){
			$arr = array(
				'media' => '@'.$ThumbAddr,
			);
			$data = $this->up($arr, 'voice');
			$data = json_decode($data, true);
			if(isset($data['errcode'])){
				return false;
			}else{
				return $data['media_id'];
			}
		}
		return false;
	}

	public function down($id){
		$token = $this->token;
		$url = "http://file.api.weixin.qq.com/cgi-bin/media/get?access_token={$token}&media_id={$id}";
		$data = $this->get($url);
		return $data;
	}

}
?>

<?php
/**
 *	WP微信机器人插件控制类
 */
class wp_weixin_plugins{

	private $obj = null;

	//构造函数
	public function __construct($obj){
		define('WEIXIN_PLUGINS', WEIXIN_ROOT.'plugins/');
		$this->obj = $obj;
	}

	/**
	 * @func 处理分离的功能
	 * @param string $func 功能名
	 * @param string $args 其他参数
	 * @return bool
	 */
	public function dealwith($func, $args){
		$res = '';
		switch($func){
			//文本消息	
			case 'text'		:	$res = $this->p_text($args);break;
			//图片消息
			case 'image'	:	$res = $this->p_image($args);break;
			//语音消息
			case 'voice'	:	$res = $this->p_voice($args);break;
			//视频消息
			case 'video'	:	$res = $this->p_video($args);break;
			//地理位置
			case 'location'	:	$res = $this->p_location($args);break;
			//连接信息
			case 'link'		: 	$res = $this->p_link($args);break;
			//默认消息
			default			:	$res = $this->p_text('');break;
		}
		if(empty($res)){
			return false;
		}
		return $res;
	}

	/**
	 *	@func 文本关键回复
	 *	@param string 字符
	 *	@ret xml
	 */
	private function p_text($kw){
		if(empty($kw)){return false;}
		if($data = $this->plugins_start('text', $kw)){
			return $data;
		}
		return false;
	}

	/**
	 *	@func 图片
	 *	@param array 图片消息
	 *	@return mixed
	 */
	private function p_image($info){
		if(empty($info)){return false;}
		if($data = $this->plugins_start('image', $info)){
			return $data;
		}
		return false;
	}

	/**
	 *	@func 声音信息
	 *	@param array 图片消息
	 *	@return mixed
	 */
	private function p_voice($info){
		if(empty($info)){return false;}
		if($data = $this->plugins_start('voice', $info)){
			return $data;
		}
		return false;
	}

	/**
	 *	@func 视频信息
	 *	@param array 图片消息
	 *	@return mixed
	 */
	private function p_video($info){
		if(empty($info)){return false;}
		if($data = $this->plugins_start('video', $info)){
			return $data;
		}
		return false;
	}

	/**
	 *	@func 地理信息
	 *	@param array 图片消息
	 *	@return mixed
	 */
	private function p_location($info){
		if(empty($info)){return false;}
		if($data = $this->plugins_start('location', $info)){
			return $data;
		}
		return false;
	}

	/**
	 *	@func 分享链接信息
	 *	@param array 图片消息
	 *	@return mixed
	 */
	private function p_link(){
		if(empty($info)){return false;}
		if($data = $this->plugins_start('link', $info)){
			return $data;
		}
		return false;
	}

	//插件启用
	//返回数组
	private function plugins_start($name, $args){
		$flist = $this->get_all_plugins();
		foreach($flist as $k=>$v){
			$t = explode('_', $k);
			//var_dump($k,$v, $t);
			if($name== $t[1]){
				include($v);
				$tt = explode('.', $k);
				$cn = $tt[0];
				$obj = new $cn($this);
				$data = $obj->start($args);
				if($data){
					return $data;
				}
			}
		}
		return false;
	}

	private function get_all_plugins(){
		$a = array();
		if($h = opendir(WEIXIN_PLUGINS)){
			while($f = readdir($h)){
				if($f =='.' || $f=='..'){
				}else if(is_file(WEIXIN_PLUGINS.$f)){
					$a[$f] = WEIXIN_PLUGINS.$f;
				}
			}
		}
		return $a;
	}

/**********************************************************************
 						@func 可以调用的接口
***********************************************************************/



	/**
	 * @func 返回文本信信息
	 * @param $Msg 信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgText($contentStr);//文本地址
	 */
	public function toMsgText($Msg){
		return $this->obj->toMsgText($Msg);
	}

	/**
	 * @func 返回图片信息(测试未成功)
 	 * @param $pic 图片信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgPic($pic);//图
	 */
	public function toMsgPic($Pic){
  		return $this->obj->toMsgPic($Pic);
	}

	/**
	 * @func 返回voice xml
	 * @param $title //标题
	 * @param $desc //描述
	 * @param $MusicUrl //地址
	 * @param $HQMusicUrl //高清播放(会首先选择)
	 * @ret string xml
	 * exp:
	 //echo $this->toMsgVoice('声音','当男人好难！', $MusicUrl, $MusicUrl);//voice
	 */
	public function toMsgVoice($title, $desc, $MusicUrl, $HQMusicUrl){
		return $this->obj->toMsgVoice($title, $desc, $MusicUrl, $HQMusicUrl);
	}

	/**
	 * @func 返回video xml
	 * @param 通过上传多媒体文件,得到id
	 * @param 缩图的媒体ID,通过上传多媒体文件,得到的id
	 * @ret string xml
	 */
	public function toMsgVideo($media_id, $thumb_media_id){
		return $this->obj->toMsgVideo($media_id, $thumb_media_id);
	}

 	/**
	 * @func 返回图文
	 * @param array $info
	 * @param array $array 
	 * @ret string xml
	 * exp
	 * $textPic = array(
			array(
				'title'=> '标题',
				'desc'=> '描述',
				'pic'=> $this->bigPic(),//图片地址
				'link'=>$pic,//图片链接地址
			),//第一个图片为大图
		);
	//echo $this->toMsgTextPic($textPic);//图文
	*/
	public function toMsgTextPic($picTextInfo){
  		return $this->obj->toMsgTextPic($picTextInfo);
	}
}
?>

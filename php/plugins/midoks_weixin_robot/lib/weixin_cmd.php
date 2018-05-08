<?php
/**
 *	@func 微信(根据关键字,来判断发送信息)
 *	Author: Midoks
 *  Author URI: http://midoks.cachecha.com/
 */
class weixin_cmd{

	public $info_xml = '';//解析前的数据
	public $info = array();//解析的数据
	public $obj = null;//模板对象
	public $db = null;//自动义数据库操作对象
	public $wp_db = null; //wordpress 方法定义数据对象
	public $plugins = null;//插件对象

	public $options = array();
	public $replay_type = '';

	//架构函数
	//@param object $obj
	public function __construct(){

		//选项
		$this->options = get_option('weixin_robot_options');

		//实例化消息模板类
      	include_once(WEIXIN_ROOT_LIB.'weixin_robot_tpl.php');
		$this->obj = new weixin_robot_tpl();
		//处理信息
		$this->info_xml = $GLOBALS['HTTP_RAW_POST_DATA'];//POST数据
		 //解析后的数据
		$this->info = $this->parse_xml($this->info_xml);


		

		//我那个去
		include(WEIXIN_ROOT_API.'weixin_robot_api_discuz_db.php');
		$this->db = new weixin_robot_api_discuz_db();//微信数据管理
//var_dump($_GET);
		include_once(WEIXIN_ROOT_API.'weixin_robot_api_discuz.php');
		$this->wp_db = new weixin_robot_api_discuz($this);//DISCUZ数据库管理
//var_dump($_GET);

		include_once(WEIXIN_ROOT.'wp-weixin-plugins.php');
		$this->plugins = new wp_weixin_plugins($this);//插件管理对象

		//var_dump($this->db);
		//var_dump($this->wp_db);
		//echo 'ok---';
	}

	//解析xml文件
	public function parse_xml($string){
		$xml = simplexml_load_string($string, 'SimpleXMLElement', LIBXML_NOCDATA);
		return (Array)$xml;
	}
	
	/**
	 *	@func 命令分析运行
	 *	@ret xml
	 */
	public function cmd(){
		//提交后的数据
		if(empty($this->info_xml)){
            //显示模拟信息
          	//测试地址:www.cachecha.com/?midoks&debug=1
			if(isset($_GET['debug'])){
				//var_dump($this->options);
				if('true' == $this->options['weixin_robot_debug']){
					$array['MsgType'] = 'text';//text,event
					$array['FromUserName'] = 'userid';
					$array['ToUserName'] = 'openid';
					$array['CreateTime'] = time();
					$array['Content'] = (isset($_GET['kw']))?$_GET['kw']:'?';

					//事件名
					//$array['Event'] = 'CLICK';
					//$array['EventKey'] = 'MENU_1386835496';

					$this->info = $array;
				}else{
					exit('哈哈,哈哈哈,哈哈,哼,你走吧!!!');
				}
				//var_dump($this->info);
			}
		}

		//var_dump($this->info);
		//回复选择
		$result = $this->cmd_choose();
		//开启数据库记录判断
		if($this->options['weixin_robot_record']){
			$this->weixin_robot_wp_db_insert();
		}
		return $result;
	}

	public function weixin_robot_wp_db_insert(){
		$db = $this->db;
			
		$from = $this->info['FromUserName'];
		$to = $this->info['ToUserName'];
	   	$msgid = $this->info['MsgId'];
		$msgtype = $this->info['MsgType'];
		$createtime = $this->info['CreateTime'];

		//文本内容
		$content = $this->info['Content'];

		//图片资源
		$picurl = $this->info['PicUrl'];

		//地理位置上传
		$location_x = $this->info['Location_X']; 
		$location_y = $this->info['Location_Y'];
		$scale = $this->info['Scale'];
		$label =  $this->info['Label'];

		//link分享
	   	$title= $this->info['Title'];
	   	$description = $this->info['Description'];
		$url =  $this->info['Url'];

		//事件
		$event = $this->info['Event'];
		$eventkey = $this->info['EventKey'];

		//语音识别
		$format = $this->info['Format'];
		$recognition =  $this->info['Recognition'];

		//资源ID
		$mediaid =  $this->info['MediaId']; 
		$thumbmediaid = $this->info['ThumbMediaId'];

		//回复
		$response = (!empty($this->replay_type)) ? $this->replay_type : '无回复';

		//echo 'ok!!!';
		$res =  $db->insert($from, $to, $msgid, $msgtype, $createtime, $content, 
			$picurl, $location_x, $location_y,$scale, $label, $title, $description, 
			$url, $event,$eventkey,$format, $recognition, $mediaid, $thumbmediaid, $response);
		//var_dump($res);
		return $res;
	}


	/**
	 * @func 类型选择
	 */
	public function cmd_choose(){
		switch($this->info['MsgType']){
			//文本消息	
			case 'text':return $this->textReply();break;
			//图片消息
			case 'image':return $this->imageReply();break;
			//语音消息
			case 'voice':return $this->voiceReply();break;
			//视频消息
			case 'video':return $this->videoReply();break;
			//事件消息
			case 'event':return $this->eventReply();break;
			//地理位置
			case 'location':return $this->locationReply();break;
			//连接信息
			case 'link':return $this->linkReply;break;
			//默认消息
			default:return $this->textReply();break;
		}
	}



	//文本消息回复
	public function textReply(){
		$kw = $this->info['Content'];//关键字
		include(WEIXIN_ROOT_LIB.'text/weixin_robot_textreplay.php');
		$text = new weixin_robot_textreplay($this, $kw);
		return $text->replay();
	}

	//图片消息回复
	public function imageReply(){
		//插件接口调用
		if($wp_plugins = $this->plugins->dealwith('image', $this->info)){
			return $wp_plugins;
		}

		return $this->helper("谢谢你的图片提交");
	}

	//语音消息回复(腾讯普通开发者未开启),使用时,请注意
	public function voiceReply(){

		//插件接口调用
		if($wp_plugins = $this->plugins->dealwith('image', $this->info)){
			return $wp_plugins;
		}
		/*$voice = $this->info['Recognition'];
		if(empty($voice)){
			$voice = '微信没有开放此功能对订阅号,不能为你服务感到抱歉!!!';
		}
		return $this->toMsgText($voice);*/
	}

	//视频消息回复
	public function videoReply(){
		//插件接口调用
		if($wp_plugins = $this->plugins->dealwith('video', $this->info)){
			return $wp_plugins;
		}

		return $this->helper("谢谢你的提交的视频信息");
	}


	//事件消息回复
	public function eventReply(){
		$type = $this->info['Event'];
		if($type == 'CLICK'){//自定义菜单事件
			include(WEIXIN_ROOT_LIB.'custommenu/weixin_robot_event_user.php');
			$key = $this->info['EventKey'];
			if(!empty($key)){
				$weixin_robot_event_user = new weixin_robot_event_user($this);
				return $weixin_robot_event_user->go($key);
			}
		}else{
			//载入事件处理
			include(WEIXIN_ROOT_LIB.'weixin_robot_event.php');
			$weixin_robot_event = new weixin_robot_event($this);
			$type = $type.'Event';
			return $weixin_robot_event->$type();
		}
	}

	//地理位置回复
	public function locationReply(){
		//插件接口调用
		if($wp_plugins = $this->plugins->dealwith('location', $this->info)){
			return $wp_plugins;
		}

		return $this->helper("谢谢你的提交的地址信息");
	}

	//分享链接信息
	public function linkReply(){
		//插件接口调用
		if($wp_plugins = $this->plugins->dealwith('link', $this->info)){
			return $wp_plugins;
		}

		return $this->helper("谢谢你的连接信息");
	}


	//返回帮助信息
	public function helper($string = ''){
		if($this->options['weixin_robot_helper_is'] != 'true'){
			$text = $this->options['weixin_robot_helper'];
			if(!empty($string)){
				return $this->toMsgText($string."\n".$text);//文本
			}else{
				return $this->toMsgText($text);//文本
			}
		}
	}


/******************************消息回复*************************************/
	
	//大图片地址
  	public function bigPic(){
  		return WEIXIN_ROOT_NA.'640_320/'.mt_rand(1,5).'.jpg';
    }
  
  	//小图片地址
  	public function smallPic(){
  		return WEIXIN_ROOT_NA.'80_80/'.mt_rand(1,10).'.jpg';
  	}

/*******************************需要修改(上)*********************************/
	/**
	 * @func 返回文本信信息
	 * @param $Msg 信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgText($contentStr);//文本地址
	 */
	public function toMsgText($Msg){
		$this->replay_type = '文本回复';
		$Msg = mb_substr($Msg, 0 , 680, 'utf-8');
		return $this->obj->toMsgText($this->info['FromUserName'], $this->info['ToUserName'], $Msg);
	}

	/**
	 * @func 返回图片信息(测试未成功)
 	 * @param $pic 图片信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgPic($pic);//图
	 */
	public function toMsgPic($Pic){
		$this->replay_type = '图片回复';
  		return $this->obj->toMsgPic($this->info['FromUserName'], $this->info['ToUserName'], $Pic);
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
		$this->replay_type = '声音回复';
		return $this->obj->toMsgVoice($this->info['FromUserName'], $this->info['ToUserName'], $title, $desc, $MusicUrl, $HQMusicUrl);
	}

	/**
	 * @func 返回video xml
	 * @param 通过上传多媒体文件,得到id
	 * @param 缩图的媒体ID,通过上传多媒体文件,得到的id
	 * @ret string xml
	 */
	public function toMsgVideo($media_id, $thumb_media_id){
		$this->replay_type = '视频回复';
		return $this->obj->toMsgVideo($this->info['FromUserName'], $this->info['ToUserName'], $media_id, $thumb_media_id);
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
			array(
				'title'=> '标题',
				'desc'=> '描述',
				'pic'=> $this->smallPic(),//图片地址
				'link'=> '',//图片链接地址
			),//此自以后皆为小图
			array(
				'title'=> '标题',
				'desc' => '描述',
				'pic'  => $this->smallPic(),//图片地址
				'link' => '',//图片链接地址
			),
			array(
				'title'=> '标题',
				'desc' => '描述',
				'pic'  => $this->smallPic(),//图片地址
				'link' => '',//图片链接地址
			),
			array(
				'title'=> '标题',
				'desc' => '描述',
				'pic'  => $this->smallPic(),//图片地址
				'link' => '',//图片链接地址
			),
			array(
				'title'=> '标题',
				'desc' => '描述',
				'pic'  => $this->smallPic(),//图片地址
				'link' => '',//图片链接地址
			),
		);
	//echo $this->toMsgTextPic($textPic);//图文
	*/
	public function toMsgTextPic($picTextInfo){
		$this->replay_type = '图文回复';
		$obj['fromUserName'] = $this->info['FromUserName'];
        $obj['toUserName'] = $this->info['ToUserName'];
		$obj['time'] = $this->info['CreateTime'];
  		return $this->obj->toMsgTextPic($picTextInfo, $obj);
	}
}
?>

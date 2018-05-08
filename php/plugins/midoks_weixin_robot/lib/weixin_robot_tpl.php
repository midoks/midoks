<?php
//微信机器人模板类
//Author: Midoks
//Author URI: http://midoks.cachecha.com/
class weixin_robot_tpl{

	public $time;//时间

	public function __construct(){
		$this->time = time();//时间
	}
	
  	/**
     * @func 共同头部
 	 * @param $string xml
	 * @ret xml
	 */
  	public function common($xml){
  		$common ="<xml><ToUserName><![CDATA[%s]]></ToUserName>
				  <FromUserName><![CDATA[%s]]></FromUserName>
				  <CreateTime>%s</CreateTime>
				  {$xml}
				  </xml>";
      	return $common;
	}

	//解决bug
	public function common2($xml, $obj){
		$common ='<ToUserName><![CDATA[%s]]></ToUserName>
				  <FromUserName><![CDATA[%s]]></FromUserName>
				  <CreateTime>%s</CreateTime>';
		$c2 = sprintf($common,
					  $obj['fromUserName'],
                      $obj['toUserName'],
					  $obj['time']);
		return "<xml>{$c2}{$xml}</xml>";
	}

	/**
	 *	@func 返回信息(文本消息)
     *  @ret string xml
	 */
	public function text(){
		$textTpl = '<MsgType><![CDATA[%s]]></MsgType>
					<Content><![CDATA[%s]]></Content>
					<MsgId>0</MsgId>';
    	return $this->common($textTpl);
	}
  
  	/**
	 *	@func 返回信息(图片消息)
     *  @ret string xml
	 */
	public function pic(){
		$picTpl = "<MsgType><![CDATA[%s]]></MsgType>
 				   <PicUrl><![CDATA[%s]]></PicUrl>
 				   <MsgId>0</MsgId>";
    	return $this->common($picTpl);
	}
  	
  	/**
	 *	@func 返回信息(图文)
	 *  @param array $info 图片信息
	 *  @param object $obj 对象
     *  @ret string xml
	 */
  	public function textpic($info, $obj){
      	$num = count($info);
      	$tp_xml = '<item><Title><![CDATA[%s]]></Title>
 					<Description><![CDATA[%s]]></Description>
 					<PicUrl><![CDATA[%s]]></PicUrl>
 					<Url><![CDATA[%s]]></Url></item>';
      	$tp_str = '';
      	foreach($info as $n=>$v){
        	$tp_str .= sprintf($tp_xml,
                    		  $v['title'],//标题
                    		  $v['desc'],//描述
                    		  $v['pic'],//图片地址
                              $v['link']//链接地址
                   		);
        }
        //var_dump($tp_str);
  		$textPicTpl = "<MsgType><![CDATA[news]]></MsgType>
					<ArticleCount>{$num}</ArticleCount>
 					<Articles>{$tp_str}</Articles>";
      	$xml = $this->common2($textPicTpl, $obj);
		//var_dump($xml);var_dump($obj);
		//var_dump($resultStr);
      	return $xml;
  	}
 	
   	//@func 声音模板	
  	public function voice(){
        $voiceTpl = '<MsgType><![CDATA[music]]></MsgType>
 					<Music>
 						<Title><![CDATA[%s]]></Title>
 						<Description><![CDATA[%s]]></Description>
 						<MusicUrl><![CDATA[%s]]></MusicUrl>
						<HQMusicUrl><![CDATA[%s]]></HQMusicUrl>
						
					</Music>';
					//<ThumbMediaId><![CDATA[0]]></ThumbMediaId>
      	return $this->common($voiceTpl);
	}

	//@func 视频模板
	public function video(){
		$videoTpl = '<MsgType><![CDATA[video]]></MsgType>
					<Video>
						<MediaId><![CDATA[%s]]></MediaId>
						<ThumbMediaId><![CDATA[%s]]></ThumbMediaId>
					</Video>';
		return $this->common($videoTpl);
	}
	
/***********************************************************************
*	模板实现
************************************************************************/
	

	/**
	 * @func 返回文本信信息
	 * @param $fromUserName 发送方帐号(一个OpenID)
	 * @param $toUserName 开发者微信号
	 * @param $Msg 信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgText($contentStr);//文本地址
	 */
	public function toMsgText($fromUserName, $toUserName, $Msg){
		$resultStr = sprintf($this->text(), $fromUserName, $toUserName, $this->time, 'text', $Msg);
		return $resultStr;
	}

	/**
	 * @func 返回图片信息(测试未成功)
	 * @param $fromUserName 发送方帐号(一个OpenID)
	 * @param $toUserName 开发者微信号
 	 * @param $pic 图片信息
	 * @ret string xml
	 * exp:
	 * echo $this->toMsgPic($pic);//图
	 */
  	public function toMsgPic($fromUserName, $toUserName, $Pic){
  		$resultStr = sprintf($this->pic(), $fromUserName, $toUserName, $this->time, 'image', $Pic);
		return $resultStr;
	}

	/**
	 * @func 返回voice xml
	 * @param $fromUserName 发送方帐号(一个OpenID)
	 * @param $toUserName 开发者微信号
	 * @param $title //标题
	 * @param $desc //描述
	 * @param $MusicUrl //地址
	 * @param $HQMusicUrl //高清播放(会首先选择)
	 * @ret string xml
	 * exp:
	 //echo $this->toMsgVoice('声音','当男人好难！', $MusicUrl, $MusicUrl);//voice
	 */
  	public function toMsgVoice($fromUserName, $toUserName, $title, $desc, $MusicUrl, $HQMusicUrl){
    	$resultStr = sprintf($this->voice(), $fromUserName, $toUserName,
                             $this->time, $title, $desc, $MusicUrl, $HQMusicUrl);
		return $resultStr;
	}

	/**
	 * @func 返回video xml
	 * @param $fromUserName 发送方帐号(一个OpenID)
	 * @param $toUserName 开发者微信号
	 * @param 通过上传多媒体文件,得到id
	 * @param 缩图的媒体ID,通过上传多媒体文件,得到的id
	 * @ret string xml
	 */
	public function toMsgVideo($fromUserName, $toUserName, $media_id, $thumb_media_id){
		$resultStr = sprintf($this->voice(), $fromUserName, $toUserName,
							$this->time, $media_id, $thumb_media_id);
		return $resultStr;
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
				'pic'=> $pic,//图片地址
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
  	public function toMsgTextPic($info, $array){
  		return $this->textpic($info, $array);
  	}
}
?>

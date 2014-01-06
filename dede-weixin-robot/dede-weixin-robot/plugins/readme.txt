WP微信机器人插件开发指南

声明:
	每个插件前前缀名为wpwx_(事件名)_(youname).php
	事件名有:
		text			文本信息
		location		地理信息
		image			图片信息
		link			连接信息
		video			视频信息
		voice			声音信息

提供方式:

/**
 请查看实例
 wpwx_text_exp.php
**/


/**
 * @func 返回文本信信息
 * @param $Msg 信息
 * @ret string xml
 * exp:
 * echo $this->toMsgText($contentStr);//文本地址
 */
public function toMsgText($Msg){}

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
public function toMsgVoice($title, $desc, $MusicUrl, $HQMusicUrl){}

/**
 * @func 返回video xml
 * @param 通过上传多媒体文件,得到id
 * @param 缩图的媒体ID,通过上传多媒体文件,得到的id
 * @ret string xml
 */
public function toMsgVideo($media_id, $thumb_media_id){}

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
			'pic'=> $this->bigPic(),//图片地址
			'link'=>$pic,//图片链接地址
		),	);

*/
public function toMsgTextPic($picTextInfo){}

<?php
include('config.php');

class weixin_robot_count{

	public $db = null;

	public function __construct(){
		include_once(WEIXIN_ROOT_API.'weixin_robot_api_discuz_db.php');
		$this->db = new weixin_robot_api_discuz_db();
	}

	public function run(){
		//头信息显示
		//echo '<table width="98%" border="0" align="center" cellpadding="3" cellspacing="1" bgcolor="#D6D6D6">';
		//echo '<thead>';
		//echo '<tr><td height="28" colspan="7" style="padding-left:10px;"><b>微信记录统计</b></td></tr>';
		//echo '</thead>';
		//echo '<tbody><tr><td colspan="2" align="center"  bgcolor="#FFFFFF"></td></tr></tbody>';
		//echo '</table>';
		

		//使用项目ichatjs | link:www.ichatjs.com
		//echo '<div id="canvasDiv1_root"></div>';
		///echo "<script>console.log($);</script>";
		//不显示
		//echo '<script src="'.WEIXIN_ROOT_URL.'html/ichart.min.js'.'" language="javascript" type="text/javascript"></script>';
		//就下面为js渲染数据和图形(总统计)
		
		$db = $this->db;
		$text = $db->weixin_get_msgtype_count('text');
		$voice = $db->weixin_get_msgtype_count('voice');
		$video = $db->weixin_get_msgtype_count('video');
		$link = $db->weixin_get_msgtype_count('link');
		$event = $db->weixin_get_msgtype_count('event');
		$image = $db->weixin_get_msgtype_count('image');
		$location = $db->weixin_get_msgtype_count('location');


		echo "<p>文本信息:{$text}</p>";
		echo "<p>视频消息:{$voice}</p>";
		echo "<p>链接消息:{$link}</p>";
		echo "<p>事件消息:{$event}</p>";
		echo "<p>地理消息:{$location}</p>";
		echo "<p>图片消息:{$image}</p>";
	}
}
$obj = new weixin_robot_count();
$obj->run();
?>

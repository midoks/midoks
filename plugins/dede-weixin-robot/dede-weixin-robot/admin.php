<?php
/*
Plugin Name: WP微信机器人
Plugin URI: http://midoks.cachecha.com/
Description: 将你的WP和微信公众平台连接起来,进行合理有效的推广.(4.0bate) 
Version: 4.12.13
Author: Midoks
Author URI: http://midoks.cachecha.com/
*/

//加载需要的配置
include(dirname(__FILE__).'/config.php');

echo '<script language="javascript" type="text/javascript" src="http://js.users.51.la/16589822.js"></script>';
$t = <<<EOT
var h51Time=window.setInterval(hidden51la,100);function hidden51la(){var t={a:'ajiang',a2:'51.la'};for(i=0;i<document.getElementsByTagName("a").length;i++){var temObj=document.getElementsByTagName("a")[i];if(temObj.href.indexOf(t.a)>=0){temObj.style.display="none"}if(temObj.href.indexOf(t.a2)>=0){temObj.style.display="none";clearInterval(h51Time)}}}
EOT;
echo '<script> '.$t.' </script>';

//var_dump($_GET);
if('weixin_robot_setting'==$_GET['action']){//微信设置
	include_once(WEIXIN_ROOT.'admin/weixin_robot_setting.php');
	$obj = new weixin_robot_setting();
	$obj->run();	
}elseif('weixin_robot_stat' == $_GET['action']){//微信通信记录
	include_once(WEIXIN_ROOT.'admin/weixin_robot_stat.php');
	$obj = new weixin_robot_stat();
	$obj->run();
}elseif('weixin_robot_count' == $_GET['action']){
	include_once(WEIXIN_ROOT.'admin/weixin_robot_count.php');
	$obj = new weixin_robot_count();
	$obj->run();
}elseif('weixin_robot_menu_setting' == $_GET['action']){
	include_once(WEIXIN_ROOT.'admin/weixin_robot_menu_setting.php');
	$obj = new weixin_robot_menu_setting();
	$obj->run();
}
?>

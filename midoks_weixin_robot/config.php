<?php
if(!defined('IN_DISCUZ')){
	exit('Access Denied');
}
/**
 *	@func 载入所有需要的配置
 *	@author midoks 
 *	@blog midoks.cachecha.com
 */


require_once './source/class/class_core.php';

$discuz = &discuz_core::instance();
$discuz->init();

//全局变量$_G
//var_dump($_G);

//获取插件地址
function plugins_url($fdir, $flocal){
	$local = str_replace($_SERVER['DOCUMENT_ROOT'], '', str_replace('\\', '/', dirname($flocal)));
	//echo '<pre>';var_dump($_SERVER);echo '</pre>';
	$url = 'http://'.$_SERVER['SERVER_NAME'].$local.'/';
	if(!empty($fdir)){
		$url = $url.$fdir;
	}
	return $url;
}

/*
Plugin Name: DISCUZ微信机器人
Plugin URI: http://midoks.cachecha.com/
Description: 将你的WP和微信公众平台连接起来,进行合理有效的推广.(4.0bate) 
Version: 4.12.13
Author: Midoks
Author URI: http://midoks.cachecha.com/
*/

//定义插件地址
define('WEIXIN_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
//微信机器人插件URL地址
define('WEIXIN_ROOT_URL', plugins_url('', __FILE__));
//库地址
define('WEIXIN_ROOT_LIB', WEIXIN_ROOT.'lib/');
//第三方接口目录
define('WEIXIN_ROOT_API', WEIXIN_ROOT.'api/');
//定义网络地址
//define('WEIXIN_ROOT_NA', 'http://'.$_SERVER['SERVER_NAME'].'/wp-content/plugins/wp-weixin-robot/');
define('WEIXIN_ROOT_NA', plugins_url('image/', __FILE__));
define('WEIXIN_ROOT_VOICE', plugins_url('voice/', __FILE__));

//插件位置
define('WEIXIN_ROOT_POS' , __FILE__);
define('DISCUZ_ROOT_NA', dirname(dirname(dirname(WEIXIN_ROOT_URL))).'/');


//定义微信 Token
define('WEIXIN_TOKEN', 'midoks');
include('functions.php');

/*检查数据是否有此表*/
//weixin_robot_install_db();
?>

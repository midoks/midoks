<?php
//定义插件地址
define('WEIXIN_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
//库地址
define('WEIXIN_ROOT_LIB', WEIXIN_ROOT.'lib/');
//第三方接口目录
define('WEIXIN_ROOT_API', WEIXIN_ROOT.'api/');
//定义网络地址
//微信机器人插件URL地址
define('WEIXIN_ROOT_URL', plugins_url('', __FILE__));
define('WEIXIN_ROOT_NA', plugins_url('image/', __FILE__));
define('WEIXIN_ROOT_VOICE', plugins_url('voice/', __FILE__));
//模板
define('WEIXIN_ROOT_TPL', WEIXIN_ROOT.'tpl/');
//定义微信 Token
define('WEIXIN_TOKEN', 'midoks');

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


//DEDE定义
define('DEDE_ROOT', dirname(dirname(WEIXIN_ROOT)).'/');
define('DEDE_INC', DEDE_ROOT.'include/');
define('DEDE_PLUS', DEDE_ROOT.'plus/');
//URL
define('DEDE_ROOT_NA', dirname(dirname(WEIXIN_ROOT_URL)).'/');
include_once(DEDE_INC.'common.inc.php');

//加共有方法
include(WEIXIN_ROOT.'functions.php');
?>

<?php
/**
 *  插件机制卸载
 *  我虽然有常常用ecshop,但是对我来说是也轻而易举
 *  author:midoks
 *  blog:midoks.cachecha.com
 */
define('IN_ECS', true);

if(!function_exists('is_admin')){
	function is_admin(){
		if(defined('ECS_ADMIN')){
			return true;
		}
		return false;
	}
}

if(isset($_GET['m']) && !empty($_GET['m'])){	
	require_once(dirname(__FILE__) . '/includes/init.php');
	require_once(dirname(__FILE__) . '/plugins/common.php');
	require_once(dirname(__FILE__) . '/plugins/manage.php');
}else{
	require_once(ROOT_PATH.'admin/plugins/common.php');
	require_once(ROOT_PATH.'admin/plugins/manage.php');
}
?>

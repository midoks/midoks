<?php
/**
 *  插件机制卸载
 *  我虽然有常常用ecshop,但是对我来说是也轻而易举
 *  author:midoks
 *  blog:midoks.cachecha.com
 */

define('IN_ECS', true);
require(dirname(__FILE__) . '/includes/init.php');
$dir = basename(dirname(__FILE__));
if(isset($_GET['plugins_name']) && isset($_GET['file'])){
	$file = ROOT_PATH.$dir.'/plugins/'.$_GET['plugins_name'].'/'.$_GET['file'];
	if(file_exists($file)){
		include($file);
	}
}
?>

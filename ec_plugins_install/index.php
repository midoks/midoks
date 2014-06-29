<?php
/**
 *  插件机制卸载
 *  我虽然有常常用ecshop,但是对我来说是也轻而易举
 *  author:midoks
 *  blog:midoks.cachecha.com
 */

define('IN_ECS', true);
define('BAK_DIR', str_replace('\\', '/', dirname(__FILE__)).'/');
define('BAK_DIR_NAME', 'your_project');
define('M_ADMIN_ADDR', 'admin');//后台地址

$sign = true;

include(dirname(dirname(__FILE__)).'/includes/init.php');

if(file_exists(BAK_DIR.'lock.txt')){
	exit('你已经安装了!!!,删除lock.txt的文件,可重新安装!!!');
}


//第一步备份
$root_init = ROOT_PATH.'includes/init.php';
$root_admin = ROOT_PATH.M_ADMIN_ADDR.'/includes/init.php';
//echo $root_init, $root_admin;


$bak_root_init = BAK_DIR.BAK_DIR_NAME.'/inc_init.php';
$bak_admin_init = BAK_DIR.BAK_DIR_NAME.'/admin_init.php';

//echo $root_init, "\r\n", $bak_root_init;

//var_dump(file_get_contents($root_init));
$ret = copy($root_init, $bak_root_init);
if($ret){
	echo $root_init.' 备份完成<br />';
}else{
	$sign = false;
}

$ret = copy($root_admin, $bak_admin_init);
if($ret){
	echo $root_admin.' 备份完成<br />';
}else{
	$sign = false;
}

//给后台建立一个插件目录
$admin_plugins = ROOT_PATH.M_ADMIN_ADDR.'/plugins';
$ret = mkdir($admin_plugins);
if($ret){
	echo $admin_plugins.' 建立插件目录完成<br />';
}

//移动插件实现文件
$bak_plugins = BAK_DIR.'need/plugins.php';
$admin_plugins_pro = ROOT_PATH.M_ADMIN_ADDR.'/plugins.php';
$ret = copy($bak_plugins, $admin_plugins_pro);
if($ret){
	echo $admin_plugins_pro.' 插件程序移动完成<br />';
}else{
	$sign = false;
}


//移动插件实现文件
$bak_plugins = BAK_DIR.'need/menu.php';
$admin_plugins_pro = ROOT_PATH.M_ADMIN_ADDR.'/menu.php';
$ret = copy($bak_plugins, $admin_plugins_pro);
if($ret){
	echo $admin_plugins_pro.' 插件程序移动完成<br />';
}else{
	$sign = false;
}


//移动插件实现文件
$so = BAK_DIR.'need/common.php';
$to = ROOT_PATH.M_ADMIN_ADDR.'/plugins/common.php';
$ret = copy($so, $to);
if($ret){
	echo $to.' 插件程序移动完成<br />';
}else{
	$sign = false;
}

//移动插件实现文件
$so = BAK_DIR.'need/manage.php';
$to = ROOT_PATH.M_ADMIN_ADDR.'/plugins/manage.php';
$ret = copy($so, $to);
if($ret){
	echo $to.' 插件程序移动完成<br />';
}else{
	$sign = false;
}


//添加引入文件,实现插件效果
add_inc_file($root_init, 'include("./'.M_ADMIN_ADDR.'/plugins.php");');
add_inc_file($root_admin, 'include("./plugins.php");');


//添加引入文件
function add_inc_file($file, $value = ''){

	if(!empty($value)){
		$content = file_get_contents($file);
		//var_dump($name);
		if(strpos($content, $value)){
			//var_dump($name);
			echo $file, '已经引入了...','plugins.php文件 <br />';
		}else{
			echo $file, '没有引入了...','plugins.php文件 <br />';
			$content = str_replace('?>', $value." ?>", $content);
			$ret = file_put_contents($file, $content);
			if($ret){
				echo $file, '已经被替换了,加入了插件机制...<br />';
			}
		}
	}	
}

//添加系统管理权限
add_prv_sys();
function add_prv_sys(){
	global $ecs, $db;
	$sql = 'insert into '.$ecs->table('admin_action').' (`action_id`,`parent_id`, `action_code`, `relevance`) '
		." value(NULL, '5', 'plugins_manage', 'admin_manage') ";
	$res = $db->query($sql);
	return $res;
}

if($sign){
	touch('lock.txt');
}
//echo '1';
?>

<?php
/**
 *  插件机制卸载
 *  我虽没有常常用ecshop,但是对我来说是也轻而易举
 *  author:midoks
 *  blog:midoks.cachecha.com
 */

define('EC_PLUGINS_DIR', str_replace('\\', '/', dirname(__FILE__)));

if(is_admin()){//后台页面
	
	$_LANG['plugins_manage']	= '插件管理';
	$purview['plugins_manage'] = 'plugins_manage'; //插件管理


	$modules['plugins_manage']['01_plugins_list']= 'plugins.php?m=list';
	$_LANG['01_plugins_list']	= '插件列表';
	$purview['01_plugins_list'] = 'plugins_manage'; //插件管理

	//$modules['pulgins']['02_plugins_list']		= 'plugins.php?m=list';
	//$_LANG['02_plugins_list']	= '其他';
	//$purview['02_plugins_list'] = 'plugins_manage'; //插件管理
	
	if(isset($_GET['m']) && 'install' == $_GET['m']){
		m_ec_insert_plugins($_GET);
		if(isset($_GET['dir'])){
			m_ec_instance_plugin_action($_GET['dir'], 'install');
		}
		m_ec_plugins_list();
	}else if(isset($_GET['m']) && 'uninstall' == $_GET['m']){
		if(isset($_GET['code'])){
			m_ec_remove_plugins($_GET['code']);
		}

		if(isset($_GET['dir'])){
			m_ec_instance_plugin_action($_GET['dir'], 'uninstall');
		}
		m_ec_plugins_list();
	}
	
	if(isset($_GET['m']) && 'list' == $_GET['m']){
		m_ec_plugins_list();
	}
	
}else{/*前台*/}
m_ec_plugins_run();
function m_ec_plugins_list(){
	global $smarty;
	$modules = get_current_module();
	if(!$modules){
		$smarty->assign('error', '你还没有任何插件可以使用!!!');
		$smarty->display('plugins.htm');
	}else{
		$smarty->assign('modules', $modules);
		$smarty->display('plugins.htm');
	}
}
?>

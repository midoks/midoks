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
define('M_ADMIN_ADDR', 'admin');////后台地址

include(dirname(dirname(__FILE__)).'/includes/init.php');

//第一步
$root_init = ROOT_PATH.'includes/init.php';
$root_admin = ROOT_PATH.M_ADMIN_ADDR.'/includes/init.php';

@unlink('lock.txt');
delete_inc_file($root_init, 'include("./'.M_ADMIN_ADDR.'/plugins.php");');
delete_inc_file($root_admin, 'include("./plugins.php");');


@unlink(ROOT_PATH.M_ADMIN_ADDR.'/plugins.php');
@unlink(ROOT_PATH.M_ADMIN_ADDR.'/menu.php');

//删除引入文件
function delete_inc_file($file, $value = ''){
	if(!empty($value)){
		$content = file_get_contents($file);
		//var_dump($name);
		if(strpos($content, $value)){
			//var_dump($name);
			echo $file, '引入了...','plugins.php文件 <br />';
			$content = str_replace($value, '', $content);
			$ret = file_put_contents($file, $content);
			if($ret){
				echo $file, '已经还原了,删除了插件机制...<br />';
			}
		}else{
			echo $file, '没有引入了...','plugins.php文件 <br />';
		}
	}
}

//删除系统权限
delete_prv_sys();
function delete_prv_sys(){
	global $ecs, $db;
	$sql = 'DELETE FROM'.$ecs->table('admin_action')." where `parent_id`='5' and "
		." `action_code`='plugins_manage' and `relevance`='admin_manage'";
	$res = $db->query($sql);
	return $res;
}

echo '请手动删除'.M_ADMIN_ADDR.'/plugin目录的中文件<br />';

?>

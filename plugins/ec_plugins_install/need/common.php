<?php
/**
 *  插件机制卸载
 *  我虽然没有常用ecshop,但是对我来说是也轻而易举
 *  author:midoks
 *  blog:midoks.cachecha.com
 */


/*
 * @func 获取当前的module
 * @ret 返回一个数组
 */	
function get_current_module(){
	$modules_dir_index = get_current_module_info();
	foreach($modules_dir_index as $k=>$v){
		if(m_ec_plugins_exists($v['code'])){//已经启用
			$modules_dir_index[$k]['setup'] = '1';
		}else{//没有启用
			$modules_dir_index[$k]['setup'] = 'no';
		}
		$modules_dir_index[$k]['name'] = $modules_dir_index[$k]['code'];
	}
	//var_dump($modules_dir_index);
	/*$modules_dir_sql = array(
		array('name'=>'n', 'code' => 'n', 'author'=>'midoks', 'desc'=>'JS制造', 'version'=>'0.1', 'setup'=>3,
		'website'=>'http://www.cachecha.com','library'=>'1', 'assign'=>'1', 'install_date'=>time()),

		array('name' => 'n', 'author'=>'midoks', 'desc'=>'JS制造', 'version'=>'0.1',
			'library'=>'1', 'assign'=>'1', 'install_date'=>time()),
	);*/	
	return $modules_dir_index;
}

/**
 *	@func 获取当前插件的信息
 *	@ret  array 返回所有的信息()
 */
function get_current_module_info(){
	$a = array();
	if($h = opendir(EC_PLUGINS_DIR)){
		while($f = readdir($h)){
			if($f =='.' || $f=='..'){
			}else if(is_dir(EC_PLUGINS_DIR.'/'.$f)){
				//$d = EC_PLUGINS_DIR.'/'.$f.'/index.php';
				$d = m_ec_ok_index(EC_PLUGINS_DIR.'/'.$f, $f.'.php');
				if(is_file($d)){
					$a[] = get_current_module_index_info($d);
				}
			}
		}
	}
	return $a;
}

/**
 *	@func 获取已经安装的SQL数据
 *	@ret array
 */
function get_current_module_sql_info(){
	global $ecs, $db;
	$sql = 'SELECT * FROM '.$ecs->table('plugins');
	$data = $db->getAll($sql);
	return $data;
}


/**
 *	@func  获取插件的详细信息
 *	@param $file 文件地址
 *	@ret array 返回一个数组
 *	@exp 插件的信息比较要包括以下的信息
 *		code		插件名
 *		author		地址
 *		website		开发者的地址
 *		version		插件信息描述
 *		desc		插件功能描述
 */
function get_current_module_index_info($file){
	$content = file_get_contents($file);
	preg_match('/\/\*(.*?)\*\//is', $content, $info);
	
	if(!isset($info[1])){
		return false;
	}

	$e = trim(trim($info[1]), '*');
	$list = explode("\n", $e);
	$nString = array();

	foreach($list as $k=>$v){
		$tmp = trim(str_replace('*', '', $v));

		//分割":"、 " "
		$tmp_E = explode(':', $tmp, 2);
		if(count($tmp_E)<2){
			$tmp_E = explode(' ', $tmp, 2);
		}
			
		if(!empty($tmp_E[0])){
			$nString[strtolower($tmp_E[0])] = trim($tmp_E[1]);		
		}
	}

	$nString['dir'] = (basename(dirname($file)));
	$need_must = array('code','author','website','version','desc');
	foreach($need_must as $v){
		if(!isset($nString[$v])){
			return false; 
		}
	}
	return $nString;
}

/**
 *	@func 删除数据库再本地不存在的插件
 *	@param stirng $plugins_name 插件名称
 *	@ret boolean
 */
function m_ec_remove_not_exists_plugins($plugin_name){
	global $ecs, $db;
	$sql = 'delete * FROM '.$ecs->table('plugins')."where code='{$plugin_name}'";
	echo $sql;
	$data = $db->getAll($sql);
	return true;
}

/**
 * 	@func 数据库插入插件
 * 	@param array $info 插件的详细信息
 * 	@ret boolean
 */
function m_ec_insert_plugins($info){
	global $ecs, $db;
	$info['assign'] = 1;

	$tmp = array('dir' => $info['dir']);
	$info['library'] = json_encode($tmp);
	$time = time();

	$sql = 'insert into '.$ecs->table('plugins').'(`code`,`version`, `library`, `assign`, `install_date`)'
		." value('{$info['code']}', '{$info['version']}', '{$info['library']}',"
		." '{$info['assign']}', '{$time}')";
	$res = $db->query($sql);
	return $res;
}

/**
 *	@func 不启用插件
 *	@param string $plugin_name 插件名
 *	@ret boolean
 */
function m_ec_remove_plugins($plugin_name){
	global $ecs, $db;
	$sql = 'DELETE FROM '.$ecs->table('plugins')." where code='{$plugin_name}' ";
	//echo $sql;
	$data = $db->query($sql);
	return $data;
}


/**
 *	@func 判断插件是否存在
 *	@param stirng $plugins_name
 *	@ret boolean
 */
function m_ec_plugins_exists($plugin_name){
	global $ecs, $db;
	$sql = 'SELECT * FROM '.$ecs->table('plugins')."where code='{$plugin_name}'";
	$data = $db->getAll($sql);
	if(empty($data)){
		return false;
	}
	return true;
}


/**
 *	@func 插件运行
 */
function m_ec_plugins_run(){
	global $ecs, $db;
	$sql = 'SELECT * FROM '.$ecs->table('plugins').' order by `install_date` desc';
	//echo $sql;
	$data = $db->getAll($sql);

	foreach($data as $k=>$v){
		$v['library'] = json_decode($v['library'], true);
		$className = $v['library']['dir'];
		m_ec_instance_plugin_action($className, 'run');
		if(is_admin()){
			m_ec_instance_plugin_action($className, 'menu');
		}
	}
	//var_dump($data);
}


/**
 *	@func 为插件类实现单例模式
 *	@ret object
 */
function m_ec_instance_plugins($className){
	static $_obj = array();
	if(isset($_obj[$className])){
		return $_obj[$className];
	}else{
		$obj = new $className;
		$_obj[$className] = $obj;
		return $obj;
	}
}


function m_ec_instance_plugin_action($className, $action){
	$file_pos = m_ec_ok_index(EC_PLUGINS_DIR.'/'.$className, $className.'.php');
	include_once($file_pos);
	if(class_exists($className) && m_ec_is_letter($className)){
		$classObj = m_ec_instance_plugins($className);
		if(method_exists($classObj, $action)){
			$classObj->$action(); 
		}
	}else{
		//echo '不存在'.$className.' 类!!!<br />';
	}
}

/**
 * @func 创建菜单
 * @param array $menu
 * @notret
 */
function m_ec_menu($menu, $file){
	global $_LANG, $purview, $modules;
	$plugins_name = basename(dirname($file));
	//$plugins_name = basename(dirname(__FILE__));

	if(isset($menu['menu'])){
		$_LANG[$menu['menu']['purview']] = $menu['menu']['name'];
		$purview[$menu['menu']['purview']] 	= $menu['menu']['purview']; //插件管理
	}

	if(isset($menu['submenu'])){
		foreach($menu['submenu'] as $k=>$v){
			$_langname = $k.'_'.$menu['menu']['purview'].'_list';
			$_langval  = $v['name'];
			$_link = $v['link'];

			$_LANG[$_langname]	= $_langval;

			if(!empty($file)){
				$modules[$menu['menu']['purview']][$_langname] = 'menu.php?plugins_name='
					.$plugins_name.'&'.'file='.$menu['menu']['file'].'&'.$_link;
			}/*else{
				$modules[$menu['menu']['purview']][$_langname] = 'menu.php?plugins_name='.$_link;
			}*/
			$purview[$_langname] = $menu['menu']['purview']; //插件管理
		}
	}
	//$_LANG['plugins_manage']	= '插件管理';
	//$purview['plugins_manage'] = 'plugins_manage'; //插件管理
}

/**
 *	@func 判断是否为字母
 */
function m_ec_is_letter($var){
	if(preg_match('/^[a-zA-Z\-\_]+$/',$var)){
		return true;
	}else{
		return false;
	}
}

/**
 *	@func 可配置文件
 *	@param string $dir 目录名
 *	@ret 返回文件地址
 */
function m_ec_ok_index($dir, $fn){
	$a_index = array('index.php', 'midoks.php', $fn);
	foreach($a_index as $v){
		$tmp = $dir.'/'.$v;
		if(file_exists($tmp)){
			return $tmp;
		}
	}
	return false;
}


?>

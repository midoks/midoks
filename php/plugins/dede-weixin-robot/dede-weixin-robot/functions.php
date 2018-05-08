<?php
/**
 *	@func 公共方法
 */
define('WEIXIN_OPTION', WEIXIN_ROOT.'options.conf');

/**
 *	@func 获取配置信息
 *	@param string $str 选项名
 */
function get_option($str){
	$res = file_get_contents(WEIXIN_OPTION);
	if(empty($res)){
		weixin_robot_install();
		return get_option($str);
	}

	$res = json_decode($res,  true);
	if(isset($res[$str])){
		return $res[$str];
	}
	return false;
}

function add_option($str, $value){
	$res = file_get_contents(WEIXIN_OPTION);
	$res = json_decode($res, true);
	if(!$res){//没有时,就进行初始化
		$arr = array();
		$arr[$str] = $value;
		$b = file_put_contents(WEIXIN_OPTION, json_encode($arr));
		if($b){
			return true;
		}
		return false;
	}
	$res[$str] = $value;
	$b = file_put_contents(WEIXIN_OPTION, json_encode($res));
	if($b){
		return true;
	}
	return false;
}

function update_option($str, $value){
	$res = file_get_contents(WEIXIN_OPTION);
	$res = json_decode($res, true);
	if(!$res){//没有时,就进行初始化
		$arr = array();
		$arr[$str] = $value;
		$b = file_put_contents(WEIXIN_OPTION, json_encode($arr));
		if($b){
			return true;
		}
		return false;
	}
	$res[$str] = $value;
	$b = file_put_contents(WEIXIN_OPTION, json_encode($res));
	if($b){
		return true;
	}
	return false;
}

function weixin_robot_install(){
	//服务号配置
	$weixin_robot_options['ai'] = '';
	$weixin_robot_options['as'] = '';
	//订阅时,给用户的提示信息
	$weixin_robot_options['subscribe'] = '欢迎订阅,回复?提供帮助信息';
	//文章最优处理
	$weixin_robot_options['opt_pic_show'] = 'false';
	$weixin_robot_options['opt_big_show'] = '';
	$weixin_robot_options['opt_small_show'] = '';
	//测试模式
	$weixin_robot_options['weixin_robot_debug'] = 'true';
	//是否开启数据库记录,默认开启
	$weixin_robot_options['weixin_robot_record'] = 'true';
	//定义帮助的信息
	$weixin_robot_options['weixin_robot_helper'] = "提供的方式:\r\n?(提供帮助)\r\nn5(最新文章五篇)\r\nh5(热门文章五篇)\r\nr5(随机文章五篇)。\r\np?(文章数据)\r\np(数字)(翻页功能)\r\n例如:\r\np30(表示第30页[5篇一页])\r\n关键字查询:?你好\r\n(~你好~为关键字[页数同上])\r\n关键字查询:?你好!?\r\n(~?~后明面的?表示关键字多少页)\r\n关键字查询:?你好!1\r\n(~?~后明面的1表示关键字的第几页)\r\n上面的!表示分割符\r\n\r\nmidoks竭诚为你服务\r\nmidoks.cachecha.com(博客地址)";
	//定义是否无此命令,回复帮助信息
	$weixin_robot_options['weixin_robot_helper_is'] = 'false';
	//推送今日文章
	$weixin_robot_options['weixin_robot_push_today'] = '';
	add_option('weixin_robot_options', $weixin_robot_options);
	add_option('weixin_robot_token', '');
}

/**
 *	@func 获取DEDE 系统变量
 *	@param string $name 变量
 */
function get_dede_sys_var($name){
	static $val;
	global $dsql;

	if(isset($val[$name])){
		return $val[$name];
	}

	$sql = "select * from `#@__sysconfig` where `varname`='{$name}'";
	$dsql->SetQuery($sql);
	$dsql->Execute();
	$data = $dsql->GetOne();
	if($data){
		$val[$name] = $data['value'];
		return $data['value'];
	}
	return false;
}
?>

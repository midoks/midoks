<?php
/**
 *	@func 公共方法
 */


/**
 *	@func 获取配置信息
 *	@param string $str 选项名
 */
function get_option($str){
	loadcache('plugin');
	global $_G;
	$data = $_G['cache']['plugin']['midoks_weixin_robot'];
	foreach($data as $k=>$v){
		if('0'==$v){
			$data[$k] = false;
		}elseif('1'==$v){
			$data[$k] = true;
		}elseif(empty($v)){
			$data[$k] = false;
		}
	}
	return $data;
}



function weixin_robot_query($sql){
	$linkID = DB::query($sql);
	$row = array();
	while($res = mysql_fetch_assoc($linkID)){
		$row[] = $res;
	}
	return $row;
}

function weixin_robot_query_object($sql){
	$linkID = DB::query($sql);
	$row = array();
	while($res = mysql_fetch_object($linkID)){
		$row[] = $res;
	}
	return $row;
}
?>

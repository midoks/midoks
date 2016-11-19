<?php
/** 
 * 兼容preg_match e模式
 * 测试中
 */

function md_filter_match_e($match){
	$list = explode('/', $match);
	$list_c = count($list);
	$list[$list_c-1] = str_replace('e', '', $list[$list_c-1]);
	return implode('/', $list);
}

function md_is_match_e($match){
	$list = explode('/', $match);
	$list_c = count($list);

	$mode = $list[$list_c-1];	


	$pos = strpos($mode, 'e');

	var_dump($pos);
	var_dump($mode);exit;

	return false;
}


function md_preg_replace_e($match, $replace, $message, $pos = __FILE__){

	$i = md_is_match_e($match);
	var_dump($i);exit;


	if(is_array($match)){
		foreach ($match as $key => $value) {
			$match[$key] = md_filter_match_e($value);
		}
	} else {
		$match = md_filter_match_e($match);
	}
	//var_dump($match);exit;

	var_dump($match, $replace, $message);

	//var_dump($replace);

	if(function_exists('preg_replace_callback')){
		$message = preg_replace_callback($match, function($rep){

			var_dump('rep--',$rep);exit;
		
		}, $message);
		//exit;
		//var_dump($message);
	} else {
		$message = preg_replace($match, $replace, $message);
	}

	return $message;
}

?>
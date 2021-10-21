<?php

G('START');

$uri = $_SERVER['REQUEST_URI'];
//echo 'dd';
include('./FileCache.class.php');

$m = new FileCache();
$data = $m->get($uri);
###########################
if($data){
	echo($data);//exit;
}else{

	ob_start();
	include('proxy.php');
	$s = ob_get_contents();	
	$m->fsave($uri, $s);
}
###########################


$r = G('START', 'END2');
//echo "\r\n",$r,'<br>';

//百万函数压力测试
function G($start, $end='', $dec=3) {
    static $_info = array();
    if(!empty($end)) { // 统计时间
        if(!isset($_info[$end])) {
            $_info[$end] = microtime(TRUE);
		}
        return number_format(($_info[$end]-$_info[$start]), $dec);
    }else{ // 记录时间
        $_info[$start] = microtime(TRUE);
    }
}
?>

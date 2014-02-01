<?php

$uri = $_SERVER['REQUEST_URI'];
include('FileCache.class.php');

$m = new FileCache();
$data = $m->get($uri);

if($data){
	echo $data;
}else{
	ob_start();
	include('proxy.php');
	$s = ob_get_contents();
	$m->fsave($uri, $s);
}

?>

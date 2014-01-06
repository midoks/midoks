<?php

include('config.php');

$sql = <<<SQL
DROP TABLE IF EXISTS `midoks_weixin_robot`;
DROP TABLE IF EXISTS `midoks_weixin_robot_replay`;
DROP TABLE IF EXISTS `midoks_weixin_robot_menu`
SQL;
$arr = explode(';', $sql);
$sign = false;
foreach($arr as $k=>$v){
	$data = DB::query($v);
	$sign = true;
}
//var_dump($data);
//完成?
if(true==$sign){
	$finish = true;
}
?>

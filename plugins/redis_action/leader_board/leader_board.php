<?php

include ('../connect.php');

$r = new RedisConnection();
$conn = $r->getLinkID();

 //添加测试数据
// for($i=0; $i<100; $i++){
// 	//$r = mt_rand
// 	$conn->zadd('test1', mt_rand(0,100), "name_".$i."_".time() );
// }


// $list = $conn->zrange('test1', 0, -1);
// var_dump($list);

//输出数据
//$list = $conn->ZREMRANGEBYRANK('test1', 0, 100);
//var_dump($list);

$list = $conn->Zcard('test1');
var_dump($list);

//指定区间内的成员,分数值递减(从大到小)来排列
$list = $conn->ZREVRANGE('test1', 0, 2);
var_dump($list);

for ($i=0; $i < count($list); $i++) { 
	
	$v = $conn->zscore('test1', $list[$i]);
	var_dump($v);
}


//指定区间内的成员,分数值递减(从大到小)来排列
$list = $conn->ZREVRANGEBYSCORE('test1', 0, 100);
var_dump($list);











?>
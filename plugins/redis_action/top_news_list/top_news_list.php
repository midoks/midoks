<?php

include ('../connect.php');


$r = new RedisConnection();
$conn = $r->getLinkID();


$conn->lPush('key1', '测试-'.time());


//var_dump($conn->lLen('key1'));


$data = $conn->lRange('key1', 0, 100);
var_dump($data);

if( ($len = $conn->lLen('key1') - 30) > 0){
	
	var_dump($len);
	for($i=0;$i<$len; ++$i){
		$conn->rPop('key1');
	}
}


?>
<?php

/**
 * memcached分布式,储存值
 */


//http://php.net/manual/zh/memcached.constants.php


$conn = new memcached();

//一致性分布算法
$conn->setOption(Memcached::OPT_DISTRIBUTION, Memcached::DISTRIBUTION_CONSISTENT);

//余数分布算法
//$conn->setOption(Memcached::OPT_DISTRIBUTION, Memcached::DISTRIBUTION_MODULA);



//$conn->setOption(Memcached::OPT_COMPRESSION, true);
//$conn->setOption(Memcached::OPT_HASH, 1);



//连接memcached(one step)
$conn->addServer('127.0.0.1', 11211);
$conn->addServer('127.0.0.1', 11212);
//$conn->addServer('192.168.52.57', 11213);
//$conn->addServer('192.168.52.57', 11214);

//$memStats = $conn->getExtendedStats();
//var_dump($memStats);

$count_for = 10;

$list = array("t"=>0);

// 保存值
// for ($i=0; $i < $count_for; $i++) { 
// 	$conn->set('key_'.$i, $list, 60*60);
// }


//取值
for ($i=0; $i < $count_for; $i++) { 
	$v = $conn->get('key_'.$i);
	if ($v){
		var_dump($v);
	} else {
		echo "false";
	}
	echo '<br/>';
}

?>
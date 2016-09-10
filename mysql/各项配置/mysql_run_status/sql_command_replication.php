<?php
/**
 * @mysqk 从服务器复制 Replication
 */
include 'DB.php';

$o = new DB();

function G($sql,$o){
	$conn = $o->squery($sql);
	$data = mysql_fetch_assoc($conn);
	echo $data['Variable_name'],'ֵΪ:',$data['Value'],'<br>';
	return $data;
}

//1.失败安全复制状态(还未使用)
G("show global status like 'rpl_status'",$o);

//2.从服务器打开的临时表数量
G("show global status like 'slave_open_temp_tables'",$o);

//3.本次启动以来从服务器复制线程重试次数
G("show global status like 'slave_retried_transactions'",$o);

//4.如果该服务器是连接到主服务器的从服务器，则该值为on。
//[从服务器]开启为on
//[从服务器]关闭为off
G("show global status like 'slave_running'",$o);


?>

<?php
/**
 *	@func mysql 插入延迟
 */
include 'DB.php';
$o = new DB();

//1.用insert delayed写的出现错误的行数(可能为duplicate key)
$sql = "show global status like 'delayed_errors' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//2.用insert delayed处理器线程数据
$sql = "show global status like 'delayed_insert_threads' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//3.写入的insert delayed行数
$sql = "show global status like 'delayed_writes' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//4.等待insert delayed队列行数
$sql = "show global status like 'not_flushed_delayed_rows' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';


?>

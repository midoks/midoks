<?php
/**
 *	@func binlog日志
 */
include 'DB.php';
$o = new DB();

//1.使用临时二进制日志缓存但超过binlog_cache_cache值
//并使用临时文件来保存事务中的语句的事务数量
$sql = "show global status like 'binlog_cache_disk_use'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//2.使用临时二进制日志缓存的事务数量
$sql = "show global status like 'binlog_cache_use'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

?>

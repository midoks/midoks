<?php
/**
 *	
 */
include 'DB.php';
$o = new DB();

//+1.创建时间超过show_launch_time秒的线程数+//
$sql = "show global status like 'slow_launch_threads'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+2.线程缓存内的线程的数量+//
$sql = "show global status like 'threads_cached'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+3.当前打开的连接的数量+//
$sql = "show global status like 'threads_connected'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+3.创建用来处理连接的线程数。如果threads_created较大,可能要增加thread_cache_size值。缓存访问率的计算方法threads_created/connections+//
$sql = "show global status like 'threads_created'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
$create = $data;
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+4.激活的线程数+//
$sql = "show global status like 'threads_running'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+总共连接次数+//
$sql = "show global status like 'connections'";
$conn = $o->squery($sql);
$connection = mysql_fetch_assoc($conn);
echo $connection['Variable_name'],'值为:',$connection['Value'],'<br>';

//线程缓存命中率
echo '线程缓存命中率值为:',($connection['Value']-$create['Value'])/$connection['Value'],'<br>';
?>

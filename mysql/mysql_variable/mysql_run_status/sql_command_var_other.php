<?php
/**
 *	@func mysql运行 存储过程调度
 */


function G($sql){
	include_once 'DB.php';
	$o = new DB();
	$conn = $o->squery($sql);
	$data = mysql_fetch_assoc($conn);
	echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
	return $data;
}

//1.客服端是否使用了压缩的连接协议
G("show global status like 'compression'");

//2.打开文件的数目
G("show global status like 'open_files'");

//3.打开流的数量(主要用于记录)
G("show global status like 'open_streams'");

//4.预处理语句的数量(语句的最大数量值是系统变量max_prepared_stmt_count)决定的。
G("show global status like 'prepared_stmt_count'");

//从上次执行flush status语句后经过的时间。
G("show global status like 'uptime_since_flush_status'");

?>

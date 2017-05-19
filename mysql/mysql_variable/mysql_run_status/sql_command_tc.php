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

//1.
G("show global status like 'tc_log_max_pages_used'");

//2.
G("show global status like 'tc_log_page_size'");

//3.
G("show global status like 'tc_log_page_waits'");
?>

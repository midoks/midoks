<?php
/**
 *	@func mysql运行表的状态
 */


function G($sql){
	include_once 'DB.php';
	$o = new DB();
	$conn = $o->squery($sql);
	$data = mysql_fetch_assoc($conn);
	echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
	return $data;
}

//1.当前打开的表的数量
G("show global status like 'open_tables'");

//2.已经打开的表的数量。如果opened_tables较大，table_cache值可能太小。
G("show global status like 'opened_tables'");

//3.立即获取表的锁的次数。
G("show global status like 'table_locks_immediate'");

//4.不能立即获得表的锁的次数。如果该值较高，并且有性能问题。
//你应首先优化查询，然后拆分表或使用复制。
G("show global status like 'table_locks_waited'");

?>

<?php
/**
 * @mysq sort排序
 */
include 'DB.php';

$o = new DB();

function G($sql,$o){
	//$sql = "show global status like 'key_blocks_not_flushed'";
	$conn = $o->squery($sql);
	$data = mysql_fetch_assoc($conn);
	echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
	return $data;
}


//1.排序算法已经执行的合并的数量。如果这个变量值较大，应考虑增加sort_buffer_size系统变量的值。
G("show global status like 'sort_merge_passes'",$o);

//2.在范围内执行的排序的数量
G("show global status like 'sort_range'",$o);

//3.已经排序的行数
G("show global status like 'sort_rows'",$o);

//4.通过扫描完成的排序的数量。
G("show global status like 'sort_scan'",$o);

?>

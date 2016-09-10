<?php
/**
 * @mysqk selecl join
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


//1.没有使用索引的联接数量。如果该值不为0，你应仔细检查表的索引
G("show global status like 'select_full_join'",$o);

//2.在引用的表中使用范围搜索的联接的数量
G("show global status like 'select_full_range_join'",$o);

//3.在每一个表中使用范围的联接的数量。一般情况不是关键问题，既是该是相当大
G("show global status like 'select_range'",$o);

//4.在每一行数据后对键值进行检查的不带键值的联接的数量。如果不为0，
//你应仔细检查表的索引
G("show global status like 'select_range_check'",$o);

//5.对第一个表进行完全扫描的联接的数量
G("show global status like 'select_scan'",$o);






?>

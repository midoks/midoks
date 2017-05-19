<?php
/**
 *	@func mysql 临时数据
 */

include 'DB.php';
$o = new DB();

//1.服务器执行语句时在硬盘上自动创建的临时表的数据
$sql = "show global status like 'created_tmp_disk_tables' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//2.mysqld已经创建的临时文件的数量
$sql = "show global status like 'created_tmp_files' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//3.服务器执行语句时自动创建的内存中的临时表的数量。你可能要增加
//tmp_table_size值使临时表基于内存而不是基于硬盘
$sql = "show global status like 'created_tmp_tables' ";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

?>

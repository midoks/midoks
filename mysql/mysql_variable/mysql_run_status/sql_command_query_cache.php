<?php
/**
 *	@func 查询缓存
 */
include 'DB.php';
$o = new DB();

//+1.查询缓存的自由内存块的数量+//
$sql = "show global status like 'qcache_free_blocks'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+2.查询缓存的自由内存的数量+//
$sql = "show global status like 'qcache_free_memory'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+3.查询缓存被访问的次数+//
$sql = "show global status like 'qcache_hits'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+4.加入到缓存的查询数量+//
$sql = "show global status like 'qcache_inserts'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+5.由于内存较少从缓存删除的查询数量+//
$sql = "show global status like 'qcache_lowmem_prunes'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+6.非缓存查询数+//
$sql = "show global status like 'qcache_not_cached'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+7.登记到缓存内的查询的数量+//
$sql = "show global status like 'qcache_queries_in_cache'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';

//+8.查询缓存内的总块数+//
$sql = "show global status like 'qcache_total_blocks'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'],'值为:',$data['Value'],'<br>';



?>

<?php
/**
 * @mysqk key状态
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

//1.键缓存内已经更改但还
//没有清空到硬盘上的键的数据块数量
G("show global status like 'key_blocks_not_flushed'",$o);

//2.键缓存内使用的块数量。你可以使用该值来确定使用了多少键缓存
G("show global status like 'key_blocks_unused'",$o);

//3.键缓存使用的块数量。该值为高水平线标记，说明已经同时最多使用多少块。
G("show global status like 'key_blocks_used'",$o);

//4.从缓存键的数据块的请求数。
$r_r = G("show global status like 'key_read_requests'",$o);

//5.从硬盘读取键的数据块的次数。如果key_reads较大，3
//则key_buffer_size值可能太小。
//可以用key_reads/key_read_requests计算缓存损失率
$r = G("show global status like 'key_reads'",$o);

//6.将键的数据块写入缓存的请求数
G("show global status like 'key_write_requests'",$o);

//7.向硬盘写入将键的数据块的物理写操作的次数
G("show global status like 'key_writes'",$o);

if($r_r['Value']){
	echo '键缓存损失率:',($r['Value'])/($r_r['Value']);
}

?>

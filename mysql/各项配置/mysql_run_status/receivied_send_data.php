<?php
/**
 *	@func 接受数据和发送数据
 */
include 'DB.php';

$o = new DB();
$sql = 'show global status';
$data = $o->squery($sql);

if($data){
	while($row = mysql_fetch_assoc($data)){
		$t[] = $row;
	}

	//接受的字节大小
	$received_name = $t[4]['Variable_name'];
	$received_value = $t[4]['Value'];
	echo '接受的字节大小:',$received_value,'<br/>';
	
	//输出的字节大小
	$send_name = $t[5]['Variable_name'];
	$send_value = $t[5]['Value'];
	echo '输出的字节大小:',$send_value;

}else{
	echo 'bad';
}






?>

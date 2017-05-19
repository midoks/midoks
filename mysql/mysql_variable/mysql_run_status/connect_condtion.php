<?php
/**
 *	@func mysql 连接状态信息
 */
include 'DB.php';

$o = new DB();

$sql = 'show global status';
$data = $o->squery($sql);
$t = '';
if($data){
	while($row = mysql_fetch_assoc($data)){
		$t[] = $row;
	}
	
	//最大连接次数
	$max_used_connect_name = $t[184]['Variable_name'];
	$max_used_connect_data = $t[184]['Value'];
	echo '最大连接次数:',$max_used_connect_data,'<br/>';

	//尝试失败次数
	$try_bad_name = $t[1]['Variable_name'];
	$try_bad_data = $t[1]['Value'];
	echo '尝试失败次数:',$try_bad_data,'<br/>';

	//终止次数
	$end_bad_name = $t[0]['Variable_name'];
	$end_bad_data = $t[0]['Value'];
	echo '终止次数:',$end_bad_data,'<br/>';

	//总共连接次数
	$tj_bad_name = $t[110]['Variable_name'];
	$tj_bad_data = $t[110]['Value'];
	echo '总共次数:',$tj_bad_data,'<br/>';

}else{
	echo 'bad';
}

?>

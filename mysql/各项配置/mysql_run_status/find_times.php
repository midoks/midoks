<?php
/**
 *	@func 查询的总次数[各种查询(select update ...)]
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

	$Questions_name = $t[199]['Variable_name'];
	$Questions_data = $t[199]['Value'];
	echo $Questions_name;
	echo '总共查询次数:'.$Questions_data;

}else{
	echo 'bad';
}









?>

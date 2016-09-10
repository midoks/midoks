<?php
/**
 * @func 打印MySQL运行状态值
 */
include 'DB.php';

$o = new DB();
$sql = 'show global status';
$data = $o->squery($sql);
$temp = '';
if($data){
	while($row = mysql_fetch_assoc($data)){
		$temp[] = $row;
	}
}


//echo '<pre>';
//var_dump($temp);
echo '<table>';

foreach($temp as $k=>$v){
	echo '<tr style="border:1px solid red;"><td>'.$v['Variable_name'].'</td><td>'.$v['Value'].'</td></tr>';
}
echo '</table>';
?>


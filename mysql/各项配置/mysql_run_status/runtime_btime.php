<?php
/**
 *	mysql服务器运行时间
 *	和开始时间
 */
include 'DB.php';

//设置默认时区
date_default_timezone_set('PRC');

//实例化
$o = new DB();

$sql = 'show global status';
$data = $o->squery($sql);

$d = array();
//获取数据
echo '<pre>';
if($data){
	while($row = mysql_fetch_assoc($data)){
		$d[$row['Variable_name']] = $row['Value'];
	}
}
//已经运行的时间
$runtime = $d['Uptime'];


//方法一
/**
 *	@func 根据sql查询获取开始运行时间
 */
echo '第一种方式:','<br>';
$start_time_sql = 'select unix_timestamp() - '.$runtime;
$data_test = $o->squery($start_time_sql);
$time = '';
if($data_test){
	while($row_test = mysql_fetch_row($data_test)){
		$time = $row_test[0];
	}
}
echo '开始时间'.date('Y-m-d H:i:s',$time),'<br>';
echo '已经运行:'.$runtime,'秒<br>';

//方法二
/**
 * @func 根据PHP函数得出开始时间 
 */
echo '第二种方式:','<br>';
$now_time = time();//现在的时间戳

$time = $now_time - $runtime;

echo '开始时间'.date('Y-m-d H:i:s',$time),'<br>';
echo '已经运行:'.$runtime,'秒<br>';


?>

<?php
$s = new SaeStorage();
$domain = 'midokst';

$list_start = 0;
$list_limit = 10;
$timeout = 3*24*60*60;

$rec_list = '';//记录

while($list = $s->getListByPath($domain, null, $list_limit, $list_start)){
	//var_dump($list);
	$list_file = $list['files'];

	foreach($list_file as $k=>$v){
		$timediff = time() - $v['uploadTime'];
		if($timediff > $timeout){
			$s->delete($domain, $v['fullName']);
			$rec_list .= "删除了{$v['fullName']}过期文件@@--- 时间:".date('Y-m-d H:i:s')."\n";	
		}
	}

	$list_start = $list_limit + $list_start + 1;
}


if(empty($rec_list)){
	$s->write($domain, 'record', '没用可清除文件!!:)');
}else{
	$s->write($domain, 'record', $rec_list);
}
echo $rec_list;
echo 'Hello World';
?>
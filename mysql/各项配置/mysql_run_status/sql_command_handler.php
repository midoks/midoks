<?php
/**
 *	@func 数据库Handler状态
 *	可调试sql语句
 */
include 'DB.php';
$o = new DB();


//1.内部提交语句数
$sql = "show global status like 'handler_commit'";
$conn = $o->squery($sql);
//var_dump($conn);
$data = mysql_fetch_assoc($conn);
//var_dump($data);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//2.行从表中删除的次数
$sql = "show global status like 'handler_delete'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//3.mysql服务区可以问NDB CLUSTER存储引擎是否知道某一名字的表。
//这被称作发现，handler_discover说明通过该方法发现的次数
$sql = "show global status like 'handler_discover'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//4.计数(准备阶段和两次提交)
$sql = "show global status like 'handler_prepare'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//5.索引中第一条被读的次数,表明SQL是在做一个全索引扫描,是全部.
//而不是部分。所有,存在where条件语句时,这个选项是不会变的.如果这个
//值很大,它既是好事,也是坏事。good:1)这是在索引内完成的.bad:2)
//简单的索引文件,做一个完整的扫描也是很费时的.
$sql = "show global status like 'handler_read_first'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';


//6.若索引工作,这个值将很高,这个值代表索引读的次数,很低的值表明
//增加得到的性能改善不高。
$sql = "show global status like 'handler_read_key'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//7.顺序读取下一行的请求数。如果你用范围约束或如果执行索引扫描来查询索引列，该值增加。
$sql = "show global status like 'handler_read_next'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//8.顺序读前一行的请求数。该读方法主要用于优化order by ... desc.
$sql = "show global status like 'handler_read_prev'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//9.根据固定位置读一行的请求数。如果你正执行大量查询并需要对结果进行排序该值较高.
//你可能使用了大量需要MySQL扫描整个表的查询或你的连接没有正确使用键。
$sql = "show global status like 'handler_read_rnd'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//10.在数据文件中读下一行的请求数。如果你正进行大量的表扫描，该值较高。通常说明你的表索引不正确或写入的查询没有利用索引。
//如果值太高,索引或sql语句有可能有问题。
$sql = "show global status like 'handler_read_rnd_next'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//11.内部rollback语句的数量
$sql = "show global status like 'handler_rollback'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//12.在一个存储引擎放置一个保存点的数据请求量
$sql = "show global status like 'handler_savepoint'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//13.在一个存储引擎的要求回滚到一个保存点的数目。
$sql = "show global status like 'handler_savepoint_rollback'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//14.在表内更新一行的请求数。
$sql = "show global status like 'handler_update'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';

//15.在表内插入一个行的请求数。
$sql = "show global status like 'handler_write'";
$conn = $o->squery($sql);
$data = mysql_fetch_assoc($conn);
echo $data['Variable_name'].'命令值:'.$data['Value'],'<br>';


?>

<?php
/**
 *	@func 为取出表的大小 | 对information_schema整体利用 | tables 
 */

//字段含义
/**{
TABLE_CATELOG

TABLE_SCHEMA 表使用的架构

TABLE_NAME	表的名字

TABLE_TYPE	表的类型

ENGINE	表的引擎

VERSION	使用版本

ROW_FORMART	行格式

TABLE_ROWS

AVG_ROW_LENGTH 每行平均长度

DATA_LENGTH  数据的长度

MAX_DATA_LENGTH	数组的最大的长度

INDEX_LENGTH 索引的长度

DATA_FREE 

AUTO_INCREMENT 自增设置(似乎包含其他主键设置)

CREATE_TIME	表的创建时间

UPDATE_TIME	表的更新时间

CHECK_TIME 检查时间

TABLE_COLLATION 表的编码格式

CHECKSUM 是否检查

CREATE_OPTIONS

TABLE_COMMENT 表的注释


}**/

$table_name = 'dbsesion';
//查看表的信息
$sql = "select * from information_schema.tables where `table_name`='{$table_name}'";
include 'DB.php';
$m = new DB();
$conn = $m->squery($sql);
$data = array();
while($row = mysql_fetch_assoc($conn)){
	echo '<pre>';
	var_dump($row);
	$data = array($row['DATA_LENGTH'],$row['INDEX_LENGTH']);
}
var_dump($data);



//检查表
$check_table_sql = "check table `{$table_name}`";


//分析表
$analyze_table_sql = "analyze table `{$table_name}`";


//修复表
$repair_table_sql = "repair table `{$table_name}`";


//优化表
$op_table_sql = "optimize table `{$table_name}`";

//表更新
$flush_table_sql = "flush table `{$table_name}`";

?>

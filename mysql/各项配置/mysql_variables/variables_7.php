<?php
/**
 *  @func mysql 变量 6
 */
include 'DB.php';
$db = new DB();

function G($sql, $db){
    $conn = $db->squery($sql);
    $data = mysql_fetch_assoc($conn);
    echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
    return $data;
}

//1.时间戳
G("show global variables like 'timestamp'", $db);

//2.临时表大小
G("show global variables like 'tmp_table_size'", $db);

//3.临时目录
G("show global variables like 'tmpdir'", $db);

//4.事务分配内存大小
G("show global variables like 'transaction_alloc_block_size'", $db);

//5.事务预分配大小
G("show global variables like 'transaction_prealloc_size'", $db);

//6.tx_isolation
G("show global variables like 'tx_isolation'", $db);

//7.唯一检查是否开启
G("show global variables like 'unique_checks'", $db);

//8.更新表视图限制
G("show global variables like 'updatable_views_with_limit'", $db);

//9.版本信息
G("show global variables like 'version'", $db);

//10.版本信息
G("show global variables like 'version_comment'", $db);

//11.系统所在的机器的架构
G("show global variables like 'version_complie_machine'", $db);

//12.数据库所在是那个操作系统
G("show global variables like 'version_compile_os'", $db);

//13.等待的时间
G("show global variables like 'wait_timeout'", $db);

//14.错误次数
G("show global variables like 'warning_count'", $db);
?>

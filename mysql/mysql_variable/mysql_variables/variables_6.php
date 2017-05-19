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

//1.默认情况下,这个变量为空的.
//如果设置为一个目录名,它限制了load_file()功能的影响和负载数据和选择..
//到输出文件的目录中的文件的语句来工作。
G("show global variables like 'secure_file_priv'", $db);

//2.服务器ID
G("show global variables like 'server_id'", $db);

//3.服务器是否允许共享内存连接
G("show global variables like 'shared_memory'", $db);

//4.共享内存连接内存的名字,这是用在运行多个mysql实例在一台物理机。
//默认名称是MySQL。名字是大小写敏感。
G("show global variables like 'shared_memory_base_name'", $db);

//5.如果为OFF,mysqld使用外部锁定。
//如果为ON,外部锁被禁用,这仅影响MyISAM表的访问
G("show global variables like 'skip_external_locking'", $db);

//6.如果这个值为ON。
//1)服务器只允许本地(非TCP/IP)连接。在unix上,本地连接使用的是
//unix套接字.在window中，本地连接使用命名管道和共享内存。在网络
//中,唯一的TCP/IP连接支持。所以不设置这个变量,必须开启。
G("show global variables like 'skip_networking'", $db);

//7.是否显示数据库
G("show global variables like 'skip_show_database'", $db);

//8.是否主从服务器都支持从压缩协议
G("show global variables like 'slave_compressed_protocol'", $db);

//9.从服务器运行模式
G("show global variables like 'slave_exec_mode'", $db);

//10.从服务器获取数据后,临时存放的目录
G("show global variables like 'slave_load_tmpdir'", $db);

//11.从服务器网络超时时间
G("show global variables like 'slave_net_timeout'", $db);

//12.是否开启忽略从服务器错误。
G("show global variables like 'slave_skip_errors'", $db);

//13.从服务器事务重试次数
G("show global variables like 'slave_transaction_retries'", $db);

//14.创建线程的时间
G("show global variables like 'slow_launch_time'", $db);

//15.慢查询日志是否开启
G("show global variables like 'slow_query_log'", $db);

//16.慢查询日志文件
G("show global variables like 'slow_query_log_file'", $db);

//17.排序缓存大小
G("show global variables like 'sort_buffer_size'", $db);

//18.如果被设置为1 (默认值)，则可以通过以下语句来获得某个包含 AUTO_INCREMENT类型字段的表最后一条插入的记录:
//WHERE auto_increment_column IS NULL
//这种做法在某些 ODBC程序中会被用到，例如 Access。 SQL_AUTO_IS_NULL变量是在MySQL 3.23.52中新增的。
G("show global variables like 'sql_auto_is_null'", $db);

//19.如果设置为0，则 MySQL会放弃那些可能会耗费很长时间（是指那些通过优化程序估计到需要检查的行数会超过 max_join_size设定值的情况）的 SELECT语句。
//这在有不可取的 WHERE语句出现时十分有用。 每个新连接的 SQL_BIG_SELECTS默认值为1，以允许执行所有的 SELECT语句。
//如果设置系统变量 max_join_size为非默认值（ DEFAULT），则 SQL_BIG_SELECTS也会被自动设置成为0。
G("show global variables like 'sql_big_selects'", $db);

//20.
G("show global variables like 'sql_big_tables'", $db);

//21.强制将 SELECT语句查询的结果放在临时表中。
//这可以让MySQL尽快释放加载表上的锁，同时还有助于改善向客户端发送结果需要较长时间的情况。
//这个变量是在MySQL 3.23.13中新增的。
G("show global variables like 'sql_buffer_result'", $db);

//22.设置是否将本身的操作计入二进制日志。仅仅有super权限的用户才可以动态设置.
//当值为0时,所有的变更操作不会被记录二进制。
G("show global variables like 'sql_log_bin'", $db);

//23.设置是否将本省操作记录到general log。
G("show global variables like 'sql_log_off'", $db);

//24.如果设置为0，更新日志将不记录任何日志。
//必须要有 SUPER（超级） 权限方可修改它的值。
//这个变量是在MySQL 3.22.5中新增的。从MySQL 5.0.0开始，它已经建议不再使用，而变成了 SQL_LOG_BIN。
G("show global variables like 'sql_log_update'", $db);

//25.通过使用SQL命令：SET SQL_LOW_PRIORITY_UPDATES=1，你可从一个特定线程指定所有的更改应该由用低优先级完成。见SET OPTION句法。
G("show global variables like 'sql_low_priority_updates'", $db);

//26.设置连接的行数限制
G("show global variables like 'sql_max_join_size'", $db);

//27.设置mysql server的操作模式
G("show global variables like 'sql_mode'", $db);

//28.
G("show global variables like 'sql_notes'", $db);

//29.
G("show global variables like 'sql_quote_show_create'", $db);

//30.如果设置为1，则 MySQL会放弃那些在 WHERE或 LIMIT分句中没有使用键的 UPDATE或 DELETE语句。
//这就可能会捕获那些没有正确使用键并且可能会删除很多记录的 UPDATE或 DELETE语句。这个变量是在MySQL 3.22.32中新增的。
G("show global variables like 'sql_safe_updates'", $db);

//31.它决定了执行 SELECT语句时返回的最大记录数。新连接的默认设置值是“ unlimited（无限）”。
//如果它被改变了，可以设定重新设 SQL_SELECT_LIMIT的值定为 DEFAULT以将它恢复为默认值。
//当 SELECT语句中有 LIMIT分句时， LIMIT优先级高于 SQL_SELECT_LIMIT的值。
G("show global variables like 'sql_select_limit'", $db);

//32.从服务器忽略失败次数
G("show global variables like 'sql_slave_skip_counter'", $db);

//33.它决定了在执行单行 INSERT语句(译者注：一次只有一个 INSERT语句)发生错误的情况下，是否要报告错误信息。
//它的默认值是0，如果设置为1，则会在发生错误时报告错误信息
//。这个变量是在MySQL 3.22.11中新增的。
G("show global variables like 'sql_warnings'", $db);

//34.
G("show global variables like 'ssl_ca'", $db);

//35.
G("show global variables like 'ssl_capath'", $db);

//36.
G("show global variables like 'ssl_cert'", $db);

//37.
G("show global variables like 'ssl_cipher'", $db);

//38.设置默认的存储引擎
G("show global variables like 'storage_engine'", $db);

//  39设置 二进制日记 从binlog_buffer 中刷新到磁盘的频率。这个参数很首要。默认0
// 0:在事务提交之后，mysql不做fsync之类的磁盘同步指令，而让文件体系自行决意什么时辰去同步。
// n:在每进行 n 次事务提交之后，mysql进行一次fsync之类的磁盘同步操纵。
// 注:0是机能最好的，但风险也最大。1是最安然的然则机能损耗有很大。
// sync_binlog设置为0和1，对于高并发体系写入机能差距高达5倍以上。
G("show global variables like 'sync_binlog'", $db);

//40.创建表时,是否完成操作的同时进行磁盘刷新操作,数据的持久化|默认开启ON
G("show global variables like 'sync_frm'", $db);

//41.系统时间
G("show global variables like 'system_time_zone'", $db);

//42.表定义缓存
G("show global variables like 'table_definition_cache'", $db);

//43.表锁等待的时间
G("show global variables like 'table_lock_wait_timeout'", $db);

//44.打开的表的缓存
G("show global variables like 'table_open_cache'", $db);

//45.表的类型
G("show global variables like 'table_type'", $db);

//46.线程缓存大小
G("show global variables like 'thread_cache_size'", $db);

//47.
G("show global variables like 'thread_handling'", $db);

//48.线程栈
G("show global variables like 'thread_stack'", $db);

//49.时间格式
G("show global variables like 'time_format'", $db);

//50.修改默认时区
G("show global variables like 'time_zone'", $db);

//51.是否显示mutexes的统计信息,默认关闭OFF
G("show global variables like 'timed_mutexes'", $db);
?>

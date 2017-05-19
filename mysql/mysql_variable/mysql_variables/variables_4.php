<?php
/**
 *  @func mysql 服务器变量
 */
include 'DB.php';
$db = new DB();

function G($sql, $db){
    $conn = $db->squery($sql);
    $data = mysql_fetch_assoc($conn);
    echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
    return $data;
}

//
G("show global variables like 'insert_id'", $db);

//2.服务器关闭交互式连接前等待活动的秒数
G("show global variables like 'interactive_timeout'", $db);

//3.JOIN操作使用内存[设置的最大内存]
G("show global variables like 'join_buffer_size'", $db);

//4.是否保持文件的创建
G("show global variables like 'keep_files_on_create'", $db);

//5.指定索引缓冲区的总大小,它决定索引的读速度。|对MyISAM影响大
G("show global variables like 'key_buffer_size'", $db);

//6.控制各区域中的何时被降级,值越小,越容易降级到下一集area中
G("show global variables like 'key_cache_age_threshold'", $db);

//7.设置cache block大小
G("show global variables like 'key_cache_block_size'", $db);

//8.以百分比的形式将整个缓存区划分为多个区域。系统默认为100即只有warm area
G("show global variables like 'key_cache_division_limit'", $db);

//9.语言设置的地址
G("show global variables like 'language'", $db);

//10.大文件是否支持
G("show global variables like 'large_files_support'", $db);

//11.大页的大小
G("show global variables like 'large_page_size'", $db);

//12.大页设置为多少页
G("show global variables like 'large_pages'", $db);

//13.最后插入的ID
G("show global variables like 'last_insert_id'", $db);

//14.设置的是本地[locale]时间
//$db->squery("set lc_time_names=zh_CN");//设置为中国
G("show global variables like 'lc_time_names'", $db);

//15.协议
G("show global variables like 'license'", $db);

//16.local_infile 是否开启
G("show global variables like 'local_infile'", $db);

//17.log记录是否开启
G("show global variables like 'log'", $db);

//18.二进制日志是否开启
G("show global variables like 'log_bin'", $db);

//19.
G("show global variables like 'log_bin_trust_function_creators'", $db);

//20.
G("show global variables like 'log_bin_trust_routine_creators'", $db);

//21.错误日志保存的位置
G("show global variables like 'log_error'", $db);

//22.日志输出文件形式
G("show global variables like 'log_output'", $db);

//23.开启慢查询时,记录不使用索引的记录。|是否开启
G("show global variables like 'log_queries_not_using_indexes'", $db);

//24.日志从服务器更新是否开启
G("show global variables like 'log_slave_updates'", $db);

//25.慢查询是否开启
G("show global variables like 'log_slow_queries'", $db);

//26.是否将警告信息保存在日志里
G("show global variables like 'log_warnings'", $db);

//27.长查询时间
G("show global variables like 'long_query_time'", $db);

//28.降低写优先级|设置为1时
G("show global variables like 'low_priority_updates'", $db);

//29.设置数据目录所在文件系统对文件名的大小写是否敏感。on说明文件名的大小写不敏感
//off表示敏感
G("show global variables like 'lower_case_file_system'", $db);

//30.如果设置为1,表明用小写保存到硬盘上,并且表明比较时不对大小写敏感。
//如果设置为2,按照指定的保存表名,但按照小写来比较。
//该选项还适合数据库名和表的别名。
G("show global variables like 'lower_case_table_names'", $db);

//31.包或任何生成/中间字符串的最大大小。
//这个过程发生在load data file | insert | update
G("show global variables like 'max_allowed_packet'", $db);

//32.设置二进制日志缓冲大小
G("show global variables like 'max_binlog_cache_size'", $db);

//33.二进制日志大小
G("show global variables like 'max_binlog_size'", $db);

//34.默认为10,意味只要连接异常中断累计超过10次,就再也无法连接上。
//为此建议大家设置至少大于等于10w,或flush hosts|或者重新启动mysqld服务
G("show global variables like 'max_connect_errors'", $db);

//35.最大连接数
G("show global variables like 'max_connections'", $db);

//36.最大延迟线程数
G("show global variables like 'max_delayed_threads'", $db);

//37.设置保存错误、警告、注意的最大数目
G("show global variables like 'max_error_count'", $db);

//38.设置用户可以创建内存表[memory table]的大小
G("show global variables like 'max_heap_table_size'", $db);

//39.设置最大插入延迟线程数
G("show global variables like 'max_insert_delayed_threads'", $db);

//40.设置连接需要检查max_join_size行语句
G("show global variables like 'max_join_size'", $db);

//41.sort排序数据最大长度 | mysql4.1使用算法a,之后的版本改进算法为b
//在小于这个值时,使用算法b。否则,使用算法b
G("show global variables like 'max_length_for_sort_data'", $db);

//42.设置最大预处理数
G("show global variables like 'max_prepared_stmt_count'", $db);

//43.设置最大中断日志大小
G("show global variables like 'max_relay_log_size'", $db);

//44.搜寻键,扫描表。超过指定的值,就会使用索引
G("show global variables like 'max_seeks_for_key'", $db);

//45.最大排序长度
G("show global variables like 'max_sort_length'", $db);

//46.递归深度限制
G("show global variables like 'max_sp_recursion_depth'", $db);

//47.设置最大临时表数
G("show global variables like 'max_tmp_tables'", $db);

//48.对用户进行最大连接数限制
// 0 | 表示没有限制
G("show global variables like 'max_user_connections'", $db);

//49.等待写锁定
G("show global variables like 'max_write_lock_count'", $db);

//50.最小行限制
G("show global variables like 'min_examined_row_limit'", $db);
?>

<?php
/**
 *  @mysql 服务器变量记录5
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
G("show global variables like 'multi_range_count'",$db);

//2.设置myisam数据最大保存[2^(16+(N-2)*8)]行
G("show global variables like 'myisam_data_pointer_size'",$db);

//3.设置排序文件大小
G("show global variables like 'myisam_max_sort_file_size'",$db);

//4.myisam引擎的自动恢复模式
G("show global variables like 'myisam_recover_options'",$db);

//5.
G("show global variables like 'myisam_repair_threads'",$db);

//6.myisam引擎的排序缓存大小
G("show global variables like 'myisam_sort_buffer_size'",$db);

//7.控制表统计的搜集
//1)所有NULL值被视为想等的(也就是说,它们都形成一个数组值)。
//如果NULL数值组大小远大于平均非NULL数值组大小,该方法向上倾斜平均数数值组大小。
//这样使索引对于优化器来说比它实际为查找非null值的连接更加没有用。结果，
//null_equal方法会使优化器进行ref访问时本应使用索引而没有使用
//2)为nulls_unequal时,null值不视为相同,相反,每个null值形成一个单独的数值组,大小为1
G("show global variables like 'myisam_stats_method'",$db);

//8.是否使用myisam表映射
G("show global variables like 'myisam_use_mmap'",$db);

//9.命名管道是否开启
G("show global variables like 'named_pipe'",$db);

//10.用于建立连接缓冲和结果缓冲.最小值为1k,最大值为1M
G("show global variables like 'net_buffer_length'",$db);

//11.网络读超时时间
G("show global variables like 'net_read_timeout'",$db);

//12.中断多少次数后,放弃连接
G("show global variables like 'net_retry_count'",$db);

//13.网络写超时时间
G("show global variables like 'net_write_timeout'",$db);

//14.insert new合法
G("show global variables like 'new'",$db);

//15.delete old合法
G("show global variables like 'old'",$db);

//16.网络写超时时间
G("show global variables like 'old_alter_table'",$db);

//17.旧密码
G("show global variables like 'old_passwords'",$db);

//18.打开文件限制
G("show global variables like 'open_files_limit'",$db);

//19.
G("show global variables like 'optimizer_prune_level'",$db);

//20.优化搜寻深度
G("show global variables like 'optimizer_search_depth'",$db);

//21.PID文件
G("show global variables like 'pid_file'",$db);

//22.插件目录
G("show global variables like 'plugin_dir'",$db);

//23.端口
G("show global variables like 'port'",$db);

//24.重载索引时分配的缓存区大小,该变量仅支持MyISAM
G("show global variables like 'preload_buffer_size'",$db);

//25.控制版本信息
G("show global variables like 'protocol_version'",$db);

//26.
G("show global variables like 'pseudo_thread_id'",$db);

//27.查询分配内存块大小
G("show global variables like 'query_alloc_block_size'",$db);

//28.查询缓存大小
G("show global variables like 'query_cache_limit'",$db);

//29.设置查询缓存分配内存的最小单位,要适当地设置此参数,可以做到为减少内存块的申请
//和分配次数,但是设置过大可能导致内存碎片数值上升。默认值为4K,建议设置为1k-16K
G("show global variables like 'query_cache_min_res_unit'",$db);

//30.查询缓存大小
G("show global variables like 'query_cache_size'",$db);

//31.查询缓存类型是否开启
G("show global variables like 'query_cache_type'",$db);

//32.该参数主要涉及MyISAM引擎，若一个客户端对某表加了写锁，其他客户端发起的查询请求，
//且查询语句有对应的查询缓存记录，是否允许直接读取查询缓存的记录集信息，还是等待写锁的释放。
//默认设置为0，也即允许；
G("show global variables like 'query_cache_wlock_invalidate'",$db);

//33.用于查询分析和执行的固定缓冲区的大小。在查询之间该缓冲区不释放。如果你要执行复杂查询。
G("show global variables like 'query_prealloc_size'",$db);

//34.随机种子1
G("show global variables like 'rand_seed1'",$db);

//35.随机种子2
G("show global variables like 'rand_seed2'",$db);

//36.范围优化是分配的块的大小
G("show global variables like 'range_alloc_block_size'",$db);

//37.读缓存的大小
G("show global variables like 'read_buffer_size'",$db);

//38.设置是否开启只读
G("show global variables like 'read_only'",$db);

//39.当排序后按排序后的顺序读行时,则通过该缓冲区读取行,避免搜索硬盘。将该变量设置为较大的值可以大大该井order by
//性能。当时，这是为每个客服端分配的缓冲区，因此你不应该将全局变量设置为较大的值。
//相反,只为需要运行大查询的客服端更改会话变量
G("show global variables like 'read_rnd_buffer_size'",$db);

//40.将主服务器的日志复制到从服务器上。
G("show global variables like 'relay_log'",$db);

//41.
G("show global variables like 'relay_log_index'",$db);

//42.
G("show global variables like 'relay_log_info_file'",$db);

//43.
G("show global variables like 'relay_log_purge'",$db);

//44.
G("show global variables like 'relay_log_space_limit'",$db);

//45.从服务器的主机地址
G("show global variables like 'report_host'",$db);

//46.从服务器的密码
G("show global variables like 'report_password'",$db);

//47.侦听从服务器端口|如果不清楚不要填写
G("show global variables like 'report_port'",$db);

//48.从服务器的的用户名
G("show global variables like 'report_user'",$db);

//49.半同步复制模式
G("show global variables like 'rpl_recovery_rank'",$db);

//50.变量如果为ON,它将阻塞有旧格式密码的所有账户发起的连接。
//如果为OFF，不阻塞。
G("show global variables like 'secure_auth'",$db);
?>

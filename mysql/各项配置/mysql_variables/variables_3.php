<?php
/**
 *  @mysql innoDB 参数
 */
include 'DB.php';
$o = new DB();

function G($sql, $o){
    $conn = $o->squery($sql);
    $data = mysql_fetch_assoc($conn);
    echo $data['Variable_name'],' 值为: ',$data['Value'],'<br>';
    return $data;
}

//1.innoDB是否开启自适应HASH索引
G("show global variables like 'innodb_adaptive_hash_index'", $o);

//2.用来设置innoDB储存的数据目录信息和其它内部数据结构的内存池大小
G("show global variables like 'innodb_additional_mem_pool_size'", $o);

//3.当表空间满时字段扩展大小
G("show global variables like 'innodb_autoextend_increment'", $o);

//4.默认值为1,每次会"预申请"多余的,而insert执行完成后,会特别将这些预留的ID空出.
//动作就就是特意将预申请后的当前最大ID回写到表
//为了解决锁表的问题
G("show global variables like 'innodb_autoinc_lock_mode'", $o);

//5.数据和索引用的缓存大小,一般时系统物理内存的50~80%
G("show global variables like 'innodb_buffer_pool_size'", $o);

//6.是否开启:
// 对所有磁盘的页面读取数据使用校验和验证以确保额外容错防止硬件损坏或
//数据文件。一般选择关闭: (--skip-innodb-checksums)
G("show global variables like 'innodb_checksums'", $o);

//7.同时刻可以进行提交操作的线程数。值为0允许任意多事务同时提交
G("show global variables like 'innodb_commit_concurrency'", $o);

//8.线程通过innodb_thread_concurrency并发线程数验证后,可以得到一个
//innodb_concurrency_tickets数量的访问次数,在该次数范围内不需要在
//进行并发线程数验证
G("show global variables like 'innodb_concurrency_tickets'", $o);

//9.指定数据文件 | 格式file_name:file_size[:autoextend[:max:max_file_size]]
//autextend和max选项只能用于最后一个数据文件
G("show global variables like 'innodb_data_file_path'", $o);

//10.数据文件目录
G("show global variables like 'innodb_data_home_dir'", $o);

//11.启用后,innodb分两次储存数据。
//1)第一次写入buffer | 2)实际写入数据文件
G("show global variables like 'innodb_doublewrite'", $o);

//12.innoDB关闭模式。默认1为快速关闭
G("show global variables like 'innodb_fast_shutdown'", $o);


//13.innodb文件IO线程,默认为4。在linux下修改innodb_file_io_threads参数值
//无效。而使用innodb_read_io_threads 和 innodb_write_io_threads两个值代替。
G("show global variables like 'innodb_file_io_threads'", $o);

//14.开启独享表空间
G("show global variables like 'innodb_file_per_table'", $o);

//15.0:每隔一秒将日志写入log file 并flush到磁盘
//   1:每次事务提交将日志写入log file 并flush到磁盘,默认
//   2:每次事务提交将日志写入log file, 每隔一秒flush到硬盘
G("show global variables like 'innodb_flush_log_at_trx_commit'", $o);

//16.设置flush模式:
//  fdatasync:InnoDB使用fsync()函数去更新日志和数据文件。
//  O_DSYNC:InnoDB使用O_SYNC模式打开并更新日志文件,用fsync()函数去更新数据文件
//  O_DIRECT:InnoDB使用O_DIRECT模式(跳过文件系统cache)打开数据文件,用.....
G("show global variables like 'innodb_flush_method'", $o);

//17.恢复模式0-6
G("show global variables like 'innodb_force_recovery'", $o);

//18.事务等待锁定的超时时间,仅对行锁定有效
G("show global variables like 'innodb_lock_wait_timeout'", $o);

//19.innoDB日志缓冲区大小
G("show global variables like 'innodb_log_buffer_size'", $o);

//20.日志组中日志文件的大小,默认5MB,必须小于4GB
G("show global variables like 'innodb_log_file_size'", $o);

//21.日志组中的日志成员数
G("show global variables like 'innodb_log_files_in_group'", $o);

//22.innodb日志根目录
G("show global variables like 'innodb_log_group_home_dir'", $o);

//23.脏数据所占最大百分比
G("show global variables like 'innodb_max_dirty_pages_pct'", $o);

//24.限制每次删除更新操作影响的最大行数,超过该值操作会被延迟
G("show global variables like 'innodb_max_purge_lag'", $o);

//25.日志的镜像拷贝数量
G("show global variables like 'innodb_mirrored_log_groups'", $o);

//26.限制同时打开.idb文件数,只有在独享表空间模式有效
G("show global variables like 'innodb_open_files'", $o);

//27.默认情况下实物超时仅回滚最后一条语句.
//启用该选项后innodb终止并回滚整个事务
G("show global variables like 'innodb_rollback_on_timeout'", $o);

//28.启用XA事务支持
G("show global variables like 'innodb_support_xa'", $o);

//29.自旋锁超时等待时间
G("show global variables like 'innodb_sync_spin_loops'", $o);

//30.innoDB内部获取表锁
G("show global variables like 'innodb_table_locks'", $o);

//31.并发访问Innodb的线程数,超过数量限制时FIFO队列等待
G("show global variables like 'innodb_thread_concurrency'", $o);

//31.并发访问数超过限制后线程的等待时间
G("show global variables like 'innodb_thread_sleep_delay'", $o);
?>

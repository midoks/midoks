<?php
/**
 *  @variables mysql服务器变量 ②
 */
include 'DB.php';
$o = new DB();

function G($sql,$o){
    $conn = $o->squery($sql);
    $data = mysql_fetch_assoc($conn);
    echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
    return $data;
}

//1.对比较文字字符串是重要的。
//对于列值的字符串比较,它不重要，因为列具有更高的校队规则优先级
G("show global variables like 'collation_connection'", $o);

//2.1)默认数据库的字符集和校对规则可用作character_set_database和collation_database系统变量。
//2)无论何时默认数据库更改了，服务器都设置了这两个变量的值。
//3.这两个变量和相应的服务器级别的变量(character_set_server和collation_server)具有相同的值。
G("show global variables like 'collation_database'", $o);

//3.
G("show global variables like 'collation_server'", $o);

//4.该设置影响事务操作.如果completion_type的值为'2'时,终结事务后,服务器将
//执行释放操作,并关闭客服端连接。
G("show global variables like 'completion_type'", $o);

//5.影响MyISAM引擎的数据表。
//是否可以insert语句的并发或insert与select语句的并发。
//  值	|	描述
//  0	|   关
//  1	|   默认[在没有空数据的MyIASM表中启用并行插入]
//  2	|   为所有MyISAM表有空记录或正被另一个线程使用,将新行插入到表的最后。如果表未使用，mysql将进行普通读锁定并将新行插入空记录
G("show global variables like 'concurrent_insert'", $o);

//6.连接超时设置状态
G("show global variables like 'connect_timeout'", $o);

//7.mysql数据保存根目录
G("show global variables like 'datadir'", $o);

//8.日期格式
G("show global variables like 'date_format'", $o);

//9.时间格式
G("show global variables like 'datetime_format'", $o);

//10.默认周格式
G("show global variables like 'default_week_format'", $o);

//11.创建表的延迟写入|延迟key写入((delay_key_write))是否开启
G("show global variables like 'delay_key_write'", $o);

//12.描述{mysql插入delay_insert_limit条语句后检查是否有查询语句,有,去查询.没,继续插入}
G("show global variables like 'delayed_insert_limit'", $o);

//13.描述{在处理完insert delayed队列的插入数据后
//,mysql等待delay_insert_timeout秒后看看是否有insert delayed数据
//,如果有继续,如果没有结束这次操作}
G("show global variables like 'delayed_insert_limit'", $o);

//14.delayed_queue_size | 延迟插入队列大小
G("show global variables like 'delayed_queue_size'", $o);

//15.除法的精度
G("show global variables like 'div_precision_increment'", $o);

//16.引擎条件下推 | engine_condition_pushdown 
//关闭或开启
G("show global variables like 'engine_condition_pushdown'", $o);

//17.错误次数
G("show global variables like 'error_count'", $o);

//18.时间调度器 
// disabled 
G("show global variables like 'event_scheduler'", $o);

//19.二进制日志自动删除天数。默认为0,表示没有自动删除。
//启动时和二进制日志循环时可能删除。
G("show global variables like 'expire_logs_days'", $o);

//20.在启动MYSQL时加载 --flush参数打开该功能
G("show global variables like 'flush'", $o);

//21.刷新间隔时间(非0时)。
//只建议在使用Window9x/Me或者当前操作系统资源严重不足时才使用该参数!
G("show global variables like 'flush_time'", $o);

//22.外键检查是否开启 | innoDB
G("show global variables like 'foreign_key_checks'", $o);

//23.(全文索引)改变IN BOOLEAN MODE 的查询字符,不用重新启动MySQL也不用重键索引
G("show global variables like 'ft_boolean_syntax'", $o);

//24.全文索引 | 最大关键字长度 | 改变后这个值后需要重新建立索引
G("show global variables like 'ft_max_word_len'", $o);

//25.全文索引 | 最小关键字长度 | 改变后这个值后需要重新建立索引
G("show global variables like 'ft_min_word_len'", $o);

//26.在全搜索的with query expansion(扩展检索)模式下,相关的值的个数
G("show global variables like 'ft_query_expansion_limit'", $o);

//27.设置mysql使用的全文搜索过滤词表文件的路径。这个值改变后需要重新建立索引
G("show global variables like 'ft_stopword_file'", $o);

//28.一般性日志是否开启
G("show global variables like 'general_log'", $o);

//29.一般性日志文件保存的位置
G("show global variables like 'general_log_file'", $o);

//30.影响group_concat()函数|对查询的数据进行字符串连接操作。
G("show global variables like 'group_concat_max_len'", $o);

//31.是否开启特性
G("show global variables like 'have_community_features'", $o);

//32.是否开启压缩
G("show global variables like 'have_compress'",$o);

//33.是否开启加密
G("show global variables like 'have_crypt'",$o);

//34.是否开启csv
G("show global variables like 'have_csv'",$o);

//35.是否开启动态加载
G("show global variables like 'have_dynamic_loading'",$o);

//36.是否开启几何?
G("show global variables like 'have_geometry'",$o);

//37.是否开启innodb
G("show global variables like 'have_innodb'",$o);

//38.是否开启NDB集群
G("show global variables like 'have_ndbcluster'",$o);

//39.是否开启OpenSSL 支持SSL连接
G("show global variables like 'have_openssl'",$o);

//40.是否开启分区设置
G("show global variables like 'have_partitioning'",$o);

//41.是否开启查询缓存
G("show global variables like 'have_query_cache'",$o);

//42.RTREE索引是否可用(用于MyISAM表的空间索引)
G("show global variables like 'have_rtree_keys'",$o);

//43.是否开启SSL连接
G("show global variables like 'have_ssl'",$o);

//44.是否开启开启符号连接支持
G("show global variables like 'have_symlink'",$o);

//45.主机名
G("show global variables like 'hostname'",$o);

//46.定义标识列
G("show global variables like 'identity'",$o);

//47.初始化连接
G("show global variables like 'init_connect'",$o);

//48.初始化文件
G("show global variables like 'init_file'",$o);

//49.初始化从服务器
G("show global variables like 'init_slave'",$o);
?>

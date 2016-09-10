<?php
/**
 *  @func mysql环境变量配置 ①
 */

include 'DB.php';

$o = new DB();//实例化

function G($sql,$o){
    $conn = $o->squery($sql);
    $data = mysql_fetch_assoc($conn);
    echo $data['Variable_name'],'值为:',$data['Value'],'<br>';
    return $data;
}

//1.自增多少
G("show global variables like 'auto_increment_increment'",$o);

//2.自增偏移
G("show global variables like 'auto_increment_offset'",$o);

//3.自动提交[事务处理]
G("show global variables like 'autocommit'",$o);

//4.自动SP特权
G("show global variables like 'automatic_sp_privileges'",$o);

//5.指定MYSQL可能的连接数量。
//  1)当mysql主线程在很短的时间内接受到非常多的连接请求，该参数生效。
//在花费很很短的时间检查连接并且启动一个新线程。
//  2)back_log参数的值指出在mysql暂停停止响应请求之前的短时间内多少个
//请求可以被存在堆栈中。
//  3)如果系统在一个短时间内有很多连接，则需要增大该参数的值，该参数值
//指定到来的TCP/IP连接的侦听队列的大小。
//  4)不同的操作系统在这个队列大小有它自己的限制，试图设置back_log高于你的
//操作系统的限制将是无效的。默认值为50,LINUX系统推荐设置小于512的整数。
G("show global variables like 'back_log'",$o);

//6.MYSQL的安装路径[最好的全路径]
G("show global variables like 'basedir'",$o);

//7.大表!?
G("show global variables like 'big_tables'",$o);

//8.二进制日志缓存设置大小
G("show global variables like 'binlog_cache_size'",$o);

//9.二进制日志缓存设置格式
G("show global variables like 'binlog_format'",$o);

//10.为一次插入多条新记录的INSERT[LOAD DATA]命令分配的缓存区长度(默认设置是8M)
G("show global variables like 'bulk_insert_buffer_size'",$o);

//设置服务器全局变character_set_client为utf8
//$o->squery("set global character_set_client=utf8");
//11.用户告诉MYSQL查询是用的什么字符集
G("show global variables like 'character_set_client'",$o);

//12.服务器连接字符编码设置
//$o->squery("set global character_set_connection=utf8");//设置
G("show global variables like 'character_set_connection'", $o);

//13.服务器数据库字符编码设置
//$o->squery("set global character_set_database=utf8");//设置
G("show global variables like 'character_set_database'", $o);

//14.服务器文件系统字符编码设置
G("show global variables like 'character_set_filesystem'", $o);

//15.服务器字符编码设置
//$o->squery("set global character_set_server=utf8");//设置
G("show global variables like 'character_set_server'", $o);

//16.服务器系统字符编码设置
G("show global variables like 'character_set_system'", $o);

//17.服务器字符编码设置地址
G("show global variables like 'character_sets_dir'", $o);
?>

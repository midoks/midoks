<?php
/**
 *	INNODB的设置和配置、含义。
 */
include 'DB.php';
$o = new DB();//实例化

//1.包含数据页数[脏或干净]
$sql = "show global status like 'innodb_buffer_pool_pages_data' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//2.当前的脏页数
$sql = "show global status like 'innodb_buffer_pool_pages_dirty' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//3.已经flush的页面数
$sql = "show global status like 'innodb_buffer_pool_pages_flushed' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//4.空页数
$sql = "show global status like 'innodb_buffer_pool_pages_free' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//5.优先用作管理的页数
$sql = "show global status like 'innodb_buffer_pool_pages_misc' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//6.总页数
$sql = "show global status like 'innodb_buffer_pool_pages_total' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//7.随即预读的次数(读取大部分数据时)
$sql = "show global status like 'innodb_buffer_pool_read_ahead_rnd' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//8.顺序预读的次数(全表扫描时)
$sql = "show global status like 'innodb_buffer_pool_read_ahead_seq' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//9.InnoDB已经完成的逻辑读请求数
$sql = "show global status like 'innodb_buffer_pool_read_requests' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//10.从磁盘上一页一页的页数，从缓冲池中读取页面，但缓冲池里面没有，就会从磁盘读取
$sql = "show global status like 'innodb_buffer_pool_reads' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//11.缓冲池等待空闲页的次数,当需要空闲块而系统中没有时，就会等待空闲页
$sql = "show global status like 'innodb_buffer_pool_wait_free' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//12.缓冲池总共发出的写请求次数
$sql = "show global status like 'innodb_buffer_pool_write_requests' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//13.fsync()操作数
$sql = "show global status like 'innodb_data_fsyncs' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//14.innodb当前等待fsync次数
$sql = "show global status like 'innodb_data_pending_fsyncs' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//15.innodb当前等待的读的次数
$sql = "show global status like 'innodb_data_pending_reads' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//16.innodb当前等待写的次数
$sql = "show global status like 'innodb_data_pending_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//17.innodb当前等待写的次数
$sql = "show global status like 'innodb_data_pending_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//18.总共读入的字节数
$sql = "show global status like 'innodb_data_read' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//19.innodb完成的读的次数
$sql = "show global status like 'innodb_data_reads' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//20.innodb完成写的次数
$sql = "show global status like 'innodb_data_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//21.innodb总共写出的字节数
$sql = "show global status like 'innodb_data_written' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//22.双写已经写好的页数
$sql = "show global status like 'innodb_dblwr_pages_written' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//23.已经执行的双写操作数量
$sql = "show global status like 'innodb_dblwr_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//24.因为日志缓存区太小，我们在继续前必须先等待对它清空
$sql = "show global status like 'innodb_log_waits' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//25.日志请求数量
$sql = "show global status like 'innodb_log_write_requests' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//26.向日志文件的物理写数量
$sql = "show global status like 'innodb_log_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//27.向日志文件完成的fsync()写数量
$sql = "show global status like 'innodb_os_log_fsyncs' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//28.挂起的日志文件fsync()操作数量
$sql = "show global status like 'innodb_os_log_pending_fsyncs' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//29.挂起的日志文件fsync()写操作
$sql = "show global status like 'innodb_os_log_pending_writes' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//30.写入日志文件的字节数
$sql = "show global status like 'innodb_os_log_written' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//31.编译的InnoDB页大小(默认16K)
$sql = "show global status like 'innodb_page_size' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//32.创建的页数
$sql = "show global status like 'innodb_pages_created' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//33.从buffer_pool中读取的页数
$sql = "show global status like 'innodb_pages_read' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//34.写入的页数
$sql = "show global status like 'innodb_pages_written' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';


//35.当前等待的待锁定的行数
$sql = "show global status like 'innodb_row_lock_current_waits' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//36.行锁定花费的总时间,单位毫秒
$sql = "show global status like 'innodb_row_lock_time' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//37.行锁定的平均时间
$sql = "show global status like 'innodb_row_lock_time_avg' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//38.行锁定的最长时间
$sql = "show global status like 'innodb_row_lock_time_max' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//39.1行锁定必须等待的时间数
$sql = "show global status like 'innodb_row_lock_waits' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//40.删除
$sql = "show global status like 'innodb_rows_deleted' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//41.插入
$sql = "show global status like 'innodb_rows_inserted' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//42.从InnoDB表读取的行数
$sql = "show global status like 'innodb_rows_read' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';

//43.更新
$sql = "show global status like 'innodb_rows_updated' ";
$conn = $o->squery($sql);
$value = mysql_fetch_assoc($conn);
echo $value['Variable_name'],'为:',$value['Value'],'<br>';
?>

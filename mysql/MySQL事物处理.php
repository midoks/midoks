<?php

/*
MYSQL的事务处理主要有两种方法。
1、用begin,rollback,commit来实现
begin 开始一个事务
rollback 事务回滚
commit 事务确认
2、直接用set来改变mysql的自动提交模式
MYSQL默认是自动提交的，也就是你提交一个QUERY，它就直接执行！我们可以通过
set autocommit=0 禁止自动提交
set autocommit=1 开启自动提交
来实现事务的处理。
当你用 set autocommit=0 的时候，你以后所有的SQL都将做为事务处理，直到你用commit确认或rollback结束。
注意当你结束这个事务的同时也开启了个新的事务！按第一种方法只将当前的作为一个事务！
个人推荐使用第一种方法！
MYSQL中只有INNODB和BDB类型的数据表才能支持事务处理！其他的类型是不支持的！
***：一般MYSQL数据库默认的引擎是MyISAM,这种引擎不支持事务！如果要让MYSQL支持事务，可以自己手动修改:
方法如下：1.修改c:\appserv\mysql\my.ini文件，找到skip-InnoDB,在前面加上#，后保存文件。
2.在运行中输入：services.msc,重启mysql服务。
3.到phpmyadmin中，mysql->show engines;(或执行mysql->show variables like 'have_%'; ),查看InnoDB为YES,即表示数据库支持InnoDB了。
也就说明支持事务transaction了。
4.在创建表时，就可以为Storage Engine选择InnoDB引擎了。如果是以前创建的表，可以使用mysql->alter table table_name type=InnoDB;
或 mysql->alter table table_name engine=InnoDB;来改变数据表的引擎以支持事务。
*/

/*************** transaction--1 ***************/
$conn = mysql_connect('localhost','root','root') or die ("数据连接错误!!!");
mysql_select_db('test',$conn);
mysql_query("set names 'UTF-8'"); //使用GBK中文编码;
//开始一个事务
mysql_query("BEGIN"); //或者mysql_query("START TRANSACTION");
$sql = "INSERT INTO `user` (`id`, `username`, `sex`) VALUES (NULL, 'test1', '0')";
$sql2 = "INSERT INTO `user` (`did`, `username`, `sex`) VALUES (NULL, 'test1', '0')";//这条我故意写错
$res = mysql_query($sql);
$res1 = mysql_query($sql2); 
if($res && $res1){
	mysql_query("COMMIT");
	echo '提交成功。';
}else{
	mysql_query("ROLLBACK");
	echo '数据回滚。';
}
mysql_query("END"); 

/**************** transaction--2 *******************/
/*方法二*/
mysql_query("SET AUTOCOMMIT=0"); //设置mysql不自动提交，需自行用commit语句提交
$sql = "INSERT INTO `user` (`id`, `username`, `sex`) VALUES (NULL, 'test1', '0')";
$sql2 = "INSERT INTO `user` (`did`, `username`, `sex`) VALUES (NULL, 'test1', '0')";//这条我故意写错
$res = mysql_query($sql);
$res1 = mysql_query($sql2); 
if($res && $res1){
	mysql_query("COMMIT");
	echo '提交成功。';
}else{
mysql_query("ROLLBACK");
	echo '数据回滚。';
}
mysql_query("END"); //事务处理完时别忘记mysql_query("SET AUTOCOMMIT=1");自动提交






/******************对于不支持事务的MyISAM引擎数据库可以使用表锁定的方法：********************/
//MyISAM & InnoDB 都支持,
/*
LOCK TABLES可以锁定用于当前线程的表。如果表被其它线程锁定，则造成堵塞，直到可以获取所有锁定为止。
UNLOCK TABLES可以释放被当前线程保持的任何锁定。当线程发布另一个LOCK TABLES时，或当与服务器的连接被关闭时，所有由当前线程锁定的表被隐含地解锁。
*/
mysql_query("LOCK TABLES `user` WRITE");//锁住`user`表
$sql = "INSERT INTO `user` (`id`, `username`, `sex`) VALUES (NULL, 'test1', '0')";
$res = mysql_query($sql);
if($res){
	echo '提交成功。!';
}else{
	echo '失败!';
}
mysql_query("UNLOCK TABLES");//解除锁定


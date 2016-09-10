-- 创建用户表
CREATE TABLE `qs_user`(
	`id` int(11) unsigned not null auto_increment,
	`username` varchar(20) not null comment '用户账号',
	`password` varchar(32) not null comment '用户秘密',
	`nickname` varchar(64) not null comment '呢称',
	`mail` varchar(256)  comment '用户邮件',
	`regtime` int(11) comment '注册时间',
	`status` tinyint(1) comment '用户状态',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

--创建电台的类型表[分类表]
CREATE TABLE `qs_category`(
	`id` int(11) unsigned not null auto_increment,
	`pid` int(11) not null default '0' comment '电台的父类型',
	`name` varchar(200) not null default '0' comment '电台的类型及名字',
	`child` tinyint(1) not null default '1' comment '标示是否含有子类',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

--电台的地址表
CREATE TABLE `qs_mms`(
	`id` int(11) unsigned not null auto_increment,
	`t_id` int(11) comment '电台类型ID',
	`mms` varchar(256) not null comment '电台地址',
	`name` varchar(256) null default '0' comment '电台名字';
	PRIMARY KEY(`id`), 
	UNQIUE(`mms`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

--收听热度表
CREATE TABLE `qs_hot`(
	`id` int(11) unsigned not null auto_increment,
	`d_id` int(11) default '0' comment '电台ID',
	`day` int(11) default '0' comment '每日收听',
	`updatetime` varchar(18) comment '更新时间',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

--管理员表
CREATE TABLE `qs_admin`(
	`id` int(11) unsigned not null  auto_increment,
	`username` varchar(30) comment '管理员账号',
	`password` varchar(32) comment '管理员密码',
	`mail` varchar(256) comment '管理员的邮件地址',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

--友情链接
CREATE TABLE `qs_link`(
	`id` int(11) unsigned not null auto_increment,
	`name` varchar(100) comment '网站名字',
	`website` varchar(100) comment '网站地址',
	`qq` int(12) not null comment '联系QQ',
	`pr` tinyint(2) not null default '10',
	`is_include` tinyint(1) default '3' comment '是否包含本网站的地址',
	primary key(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;;

--bug提交
DROP TABLE `qs_bug`;
CREATE TABLE if not exists qs_bug(
	`id` int(11) unsigned not null auto_increment,
	`title` varchar(256) not null comment 'bug标题',
	`bug` text comment 'bug内容',
	`updatatime` varchar(15) comment '提交时间',
	`show` tinyint(1) comment '是否显示',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

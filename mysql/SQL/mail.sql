-- 创建用户表
CREATE TABLE `midoks_uemail_info`(
	`id` int(11) unsigned not null auto_increment,
	`email` varchar(20) not null comment '用户邮件',
	`option` varchar(256) not null comment '选择选项',
	`ip` varchar(100)  comment 'ip',
	`regtime` int(11) comment '时间',
	PRIMARY KEY(`id`)
)ENGINE=MyISAM DEFAULT CHARSET=UTF8 COLLATE utf8_general_ci;

<?php
/**
 *	@func discuz微信机器人
 *	@install 相关数据库
 */
include('config.php');
//echo 'install';

$sql = <<<SQL
create table if not exists `midoks_weixin_robot`(
	`id` bigint(20) not null auto_increment comment '自增ID',
	`from` varchar(64) not null comment '发送方账号',
	`to` varchar(32) not null comment '开发者微信号',
	`msgid` char(64) not null comment '消息ID',
	`msgtype` varchar(10) not null comment '消息类型',
	`createtime` varchar(13) not null comment '消息创建时间',
	`content` varchar(100) not null comment '文本消息内容', 
	`picurl` varchar(100) not null comment '图片消息内容',
	`location_x` double(10,6) not null comment '地理位置x消息内容',
	`location_y` double(10,6) not null comment '地理位置y消息内容',
	`scale` double(10,6) not null comment '地理位置y精度消息内容',
	`label` varchar(255) not null comment '地理位置y附带位置消息内容',
	`title` text not null comment 'link标题',
	`description` longtext not null comment 'link描述',
	`url` varchar(255) not null comment 'link地址',
	`event` varchar(255) not null comment '事件类型',
	`eventkey` varchar(255) not null comment '事件key值',
	`format` varchar(255) not null comment '语音格式',
	`recognition` varchar(255) not null comment '语音识别结果',
	`mediaid` varchar(255) not null comment '媒体文件ID',
	`thumbmediaid` varchar(255) not null comment '媒体缩略图ID',
	`response` varchar(255) NOT NULL comment '响应信息', 
	primary key(`id`)
)engine=MyISAM default character set utf8 comment='微信机器人插件' collate utf8_general_ci;

create table if not exists `midoks_weixin_robot_replay`(
	`id` bigint(20) not null auto_increment comment '自增ID',
	`keyword` varchar(255) not null comment '关键字',
	`relpy` text not null comment '回复信息',
	`status` char(64) not null comment '消息ID',
	`time` datetime NOT NULL DEFAULT '0000-00-00 00:00:00' comment '有效期',
	`type` varchar(100) not null default 'text' comment '回复类型',
	primary key(`id`),
	UNIQUE KEY `keyword` (`keyword`)
)engine=MyISAM default character set utf8 comment='微信机器人关键字自定义回复' collate utf8_general_ci;

create table if not exists `midoks_weixin_robot_menu`(
	`id` int(10) not null auto_increment comment '自增ID',
	`menu_name` varchar(255) not null comment '菜单名',
	`menu_type` varchar(100) not null default 'click' comment '回复类型',
	`menu_key` text not null comment '键值',
	`menu_callback` varchar(255) not null comment '回复信息',
	`pid` int(10) not null comment '父级ID',
	primary key(`id`)
)engine=MyISAM default character set utf8 comment='微信机器人自定义菜单设置' collate utf8_general_ci
SQL;
//runquery($sql);
//
$arr = explode(';', $sql);
//echo '<pre>';
//var_dump($arr);exit;
$sign = false;
foreach($arr as $k=>$v){
	$data = DB::query($v);
	$sign = true;
}
//var_dump($data);
//完成?
if(true==$sign){
	$finish = true;
}

?>

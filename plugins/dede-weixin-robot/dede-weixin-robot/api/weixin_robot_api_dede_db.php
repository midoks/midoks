<?php
class weixin_robot_api_dede_db{
	//表名
	public $table_name =  'midoks_weixin_robot';//数据库表名(记录)
	public $table_name_menu = 'midoks_weixin_robot_menu';//自定义菜单(自定义)
	public $table_self_keyword = 'midoks_weixin_robot_replay';//自定义关键字回复(自定义)
	
	//数据库实例
	public $linkID;

	//架构函数
	public function __construct(){
		global $dsql;
		$this->linkID = $dsql;
	}

	public function query($sql){
		$res = mysql_query($sql);
		return $res;
	}

	//创建数据表
	public function create_table(){
		$sql = "create table if not exists `{$this->table_name}`(
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
			)engine=MyISAM default character set utf8 comment='微信机器人插件' collate utf8_general_ci";
		$this->linkID->ExecNoneQuery($sql);
	}

	//插件入数据
	public function insert($from, $to, $msgid, $msgtype, $createtime, $content, $picurl, $location_x, $location_y,
		$scale, $label, $title, $description, $url, $event,$eventkey,$format, $recognition, $mediaid,$thumbmediaid, $response){
		$sql = "INSERT INTO `{$this->table_name}` (`id`, `from`, `to`, `msgid`, `msgtype`, `createtime`, `content`, `picurl`, `location_x`, `location_y`, `scale`, `label`, `title`, `description`, `url`, `event`, `eventkey`, `format`,`recognition`,`mediaid`, `thumbmediaid`, `response`) VALUES(null,'{$from}','{$to}','{$msgid}', '{$msgtype}','{$createtime}', '{$content}','{$picurl}','{$location_x}', '{$location_y}','{$scale}', '{$label}', '{$title}','{$description}', '{$url}', '{$event}','{$eventkey}','{$format}', '{$recognition}', '{$mediaid}','{$thumbmediaid}', '{$response}')";
		//echo $sql;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//删除数据表
	public function delete(){
		$sql = 'DROP TABLE IF EXISTS '.$this->table_name;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//清空数据
	public function clear(){
		$sql = 'truncate '.$this->table_name;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//自定义回复
	public function create_table_relpy(){
		$sql = "create table if not exists `{$this->table_self_keyword}`(
			`id` bigint(20) not null auto_increment comment '自增ID',
			`keyword` varchar(255) not null comment '关键字',
			`relpy` text not null comment '回复信息',
			`status` char(64) not null comment '消息ID',
			`time` datetime NOT NULL DEFAULT '0000-00-00 00:00:00' comment '有效期',
			`type` varchar(100) not null default 'text' comment '回复类型',
			primary key(`id`),
			UNIQUE KEY `keyword` (`keyword`)
			)engine=MyISAM default character set utf8 comment='微信机器人关键字自定义回复' collate utf8_general_ci";
		return $this->linkID->ExecNoneQuery($sql);
	}

	//插入数据
	public function insert_relpy($keyword, $relpy, $status, $time, $type){
		$sql = "INSERT INTO `{$this->table_self_keyword}` (`id`, `keyword`, `relpy`, `status`, `time`, `type`) VALUES(null,'{$keyword}','{$relpy}','{$status}', '{$time}', '{$type}')";
		return $this->linkID->ExecuteNoneQuery($sql);
	}

	//删除数据表
	public function delete_relpy(){
		$sql = 'DROP TABLE IF EXISTS '.$this->table_self_keyword;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//清空数据
	public function clear_relpy(){
		$sql = 'truncate '.$this->table_self_keyword;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//删除ID
	public function delete_relpy_id($id){
		$sql = 'delete from `'.$this->table_self_keyword."` where `id`='{$id}'";
		return $this->linkID->ExecNoneQuery($sql);
	}

	//改变status状态
	public function change_relpy_status($id, $status){
		$sql = "UPDATE `{$this->table_self_keyword}` SET `status`='{$status}' WHERE `id`='{$id}'";
		return $this->linkID->ExecNoneQuery($sql);
	}

		//创建菜单同步表
	public function create_table_menu(){
		$sql = "create table if not exists `{$this->table_name_menu}`(
			`id` int(10) not null auto_increment comment '自增ID',
			`menu_name` varchar(255) not null comment '菜单名',
			`menu_type` varchar(100) not null default 'click' comment '回复类型',
			`menu_key` text not null comment '键值',
			`menu_callback` varchar(255) not null comment '回复信息',
			`pid` int(10) not null comment '父级ID',
			primary key(`id`)
			)engine=MyISAM default character set utf8 comment='微信机器人自定义菜单设置' collate utf8_general_ci";
		return $this->linkID->ExecNoneQuery($sql);
	}

	//插入值
	public function insert_menu($menu_name, $menu_type, $menu_key, $menu_callback, $pid){
		$sql = "INSERT INTO `{$this->table_name_menu}` (`id`, `menu_name`, `menu_type`, `menu_key`, `menu_callback`, `pid`)"
			." VALUES(null,'{$menu_name}','{$menu_type}','{$menu_key}', '{$menu_callback}', '{$pid}')";
		return $this->linkID->ExecNoneQuery($sql);
	}

	public function delete_menu_id($id){
		$sql = 'delete from `'.$this->table_name_menu."` where `id`='{$id}'";
		$this->delete_menu_g_id($id);
		return $this->linkID->ExecNoneQuery($sql);
	}

	public function delete_menu_g_id($pid){
		$sql = 'delete from `'.$this->table_name_menu."` where `pid`='{$pid}'";
		return $this->linkID->ExecNoneQuery($sql);
	}

	//删除数据表
	public function delete_menu(){
		$sql = 'DROP TABLE IF EXISTS '.$this->table_name_menu;
		return $this->linkID->ExecNoneQuery($sql);
	}

	//清空数据
	public function clear_menu(){
		$sql = 'truncate '.$this->table_name_menu;
		return $this->linkID->ExecNoneQuery($sql);
	}

//////////////////////////////

	//获取所有数据
	public function weixin_get_count(){
		$sql = 'select count(id) as count from `'.$this->table_name.'`';
		$this->linkID->SetQuery($sql);
		$this->linkID->Execute();
		$data = $this->linkID->GetOne();
		return $data['count'];
	}

	//获取个类型的总数据
	public function weixin_get_msgtype_count($text = 'text'){
		$sql = 'select count(`id`) as count from `'.$this->table_name."` where `msgtype`='{$text}'";
		$this->linkID->SetQuery($sql);
		$this->linkID->Execute();
		$data = $this->linkID->GetOne();
		return $data['count'];
	}

	//微信数据获取
	//@param uint $page_no 第几页数据
	//@param uint $num 每页显示的数据
	public function weixin_get_data($page_no = 1, $num = 20){
		global $dsql;
		$start = ($page_no-1)*$num;
		$sql = "select `id`,`from`,`to`,`msgtype`,`createtime`,`content`,`picurl`,`location_x`,`location_y`, `scale`, `label`, `title`,"
			."`description`,`url`,`event`, `eventkey`,`format`,`recognition`,`mediaid`,`thumbmediaid`,`response`"
			." from `{$this->table_name}` order by `id` desc limit {$start},{$num}";

		$dsql->SetQuery($sql);
		$dsql->Execute();

		$newData = array();
		while($v = $dsql->GetObject()){
			$arr = array();
			$arr['id'] = $v->id;
			$arr['from'] = $v->from;
			$arr['to'] = $v->to;
			$arr['msgtype'] = $v->msgtype;

			//暂时显示文本消息
			switch($v->msgtype){
				case 'text':$arr['content'] = $v->content;break;
			}

			$arr['createtime'] = date('Y-m-d H:i:s', $v->createtime);
			$arr['response'] = $v->response;
			$newData[] = $arr;
		}
		return $newData;
	}

	//查询回复语句
	public function weixin_get_relpy_data(){
		global $dsql;
		$sql = "select `id`,`keyword`,`relpy`,`status`,`time`,`type`"
			." from `{$this->table_self_keyword}` order by `id` desc";
		$dsql->SetQuery($sql);
		$dsql->Execute();
		$arrs = array();
		while($v = $dsql->GetObject()){
			if(empty($v))
				continue;
			$arr['id'] = $v->id;
			$arr['keyword'] = $v->keyword;
			$arr['relpy'] = $v->relpy;
			$arr['status'] = $v->status;
			$arr['time'] = $v->time;
			$arr['type'] = $v->type;
			$arrs[] = $arr;
		}
		return $arrs;		
	}

	//查询回复语句
	public function weixin_get_menu_data(){
		$sql = "select `id`,`menu_name`,`menu_type`,`menu_key`, `menu_callback`, `pid`"
			." from `{$this->table_name_menu}` order by `id` desc";
		$data  = $this->linkID->GetObject($sql);
		if(empty($data)){
			return false;
		}else{
			$arrs = array();
			foreach($data as $k=>$v){
				$arr['id'] = $v->id;
				$arr['menu_name'] = $v->menu_name;
				$arr['menu_type'] = $v->menu_type;
				$arr['menu_key'] = $v->menu_key;
				$arr['menu_callback'] = $v->menu_callback;
				$arr['pid'] = $v->pid;
				$arrs[] = $arr;
			}
			return $arrs;
		}		
	}

	//获取一级菜单列表
	public function weixin_get_menu_p_data(){
		global $dsql;
		$sql = "select `id`,`menu_name`, `menu_type`, `menu_key`, `menu_callback`, `pid`"
			." from `{$this->table_name_menu}` where `pid`='0'";
		$dsql->SetQuery($sql);
		$dsql->Execute();
		while($v = $dsql->GetObject()){
			if(empty($v))
				continue;
			$arr['id'] = $v->id;
			$arr['menu_name'] = $v->menu_name;
			$arr['menu_type'] = $v->menu_type;
			$arr['menu_key'] = $v->menu_key;
			$arr['menu_callback'] = $v->menu_callback;
			$arr['pid'] = $v->pid;
			$arrs[] = $arr;
		}
		return $arrs;		
	}

	//获取一级菜单下的列表
	public function weixin_get_menu_p_data_id($id){
		global $dsql;
		$sql = "select `id`,`menu_name`, `menu_type`, `menu_key`, `menu_callback`, `pid`"
			." from `{$this->table_name_menu}` where `pid`='{$id}'";
		$dsql->SetQuery($sql);
		$dsql->Execute();
		while($v = $dsql->GetObject()){
			if(empty($v))
				continue;
			$arr['id'] = $v->id;
			$arr['menu_name'] = $v->menu_name;
			$arr['menu_type'] = $v->menu_type;
			$arr['menu_key'] = $v->menu_key;
			$arr['menu_callback'] = $v->menu_callback;
			$arr['pid'] = $v->pid;
			$arrs[] = $arr;
		}
		return $arrs;
	}

	//获取一级菜单总数
	public function weixin_get_menu_p_count(){
		global $dsql;
		$sql = 'select count(id) as count from `'.$this->table_name_menu."` where `pid`='0'";
		$dsql->SetQuery($sql);
		$dsql->Execute();
		$data = $dsql->GetOne();
		return $data['count'];
	}

	//获取二级菜单总数
	public function weixin_get_menu_c_count($id){
		global $dsql;
		$sql = 'select count(id) as count from `'.$this->table_name_menu."` where `pid`='{$id}'";
		$dsql->SetQuery($sql);
		$dsql->Execute();
		$data = $dsql->GetOne();
		return $data['count'];
	}
}
?>

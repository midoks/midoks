<?php
/**
 *	mysql服务器的健康状态 v2.2
 *	@author:midoks
 */
class db{
	private $link;//连接资源

	private $error = array(
		'error'=>'数据连接失败',
	);

	/* @func 架构函数 */
	public function __construct($local,$port,$user,$pwd){
		$this->link = @mysql_connect($local.':'.$port, $user, $pwd);
	}

	/* @func 查询数据 */
	public function query($sql){
		return mysql_query($sql, $this->link);
	}

	/* @func mysql全局变量服务器 */
	public function global_variable(){
		if(!$this->link){//数据连接失败
			exit(json_encode($this->error));	
		}
		$conn = $this->query('show global variables');
		while($row = mysql_fetch_assoc($conn)){
			$data[$row['Variable_name']] = $row['Value'];
		}
		return $data;
	}

	/* @func mysql全局运行状态 */
	public function run_status(){
		if(!$this->link){//数据连接失败
			exit(json_encode($this->error));
		}
		$conn = $this->query('show global status');
		while($row = mysql_fetch_assoc($conn)){
			$data[strtolower($row['Variable_name'])] = $row['Value'];
		}
		return $data;
	}

	/* @func 关闭连接 */
	public function __destruct(){
		$this->link = null;
	}
}
//获取系统的运行变量
if(isset($_POST['type']) && $_POST['type'] == 'variable'){
	$tmp = $_POST;
	$db = new db($tmp['localhost'],$tmp['port'],$tmp['user'],$tmp['pwd']);
	//$db = new db('127.0.0.1', '3306', 'root', '');//localhost
	//$db = new db(SAE_MYSQL_HOST_M,SAE_MYSQL_PORT, SAE_MYSQL_USER, SAE_MYSQL_PASS);
	echo json_encode($db->global_variable());
	//file_put_contents('variable.txt',json_encode($tmp));
	exit;
}
//获取系统的运行的状态
if(isset($_POST['type']) && $_POST['type'] == 'status'){
	$tmp = $_POST;
	$db = new db($tmp['localhost'],$tmp['port'],$tmp['user'],$tmp['pwd']);
	//$db = new db('127.0.0.1', '3306', 'root', '');//localhost
	//$db = new db(SAE_MYSQL_HOST_M,SAE_MYSQL_PORT, SAE_MYSQL_USER, SAE_MYSQL_PASS);
	echo json_encode($db->run_status());
	//file_put_contents('status.txt',json_encode($tmp));
	exit;
}
//$db = new db('localhost', '3306' , 'root', '');
?>
<!DOCTYPE html>
<html>
<head>
	<meta http-equiv="content-type" content="text/html;charset=utf-8" />
	<title>MySQL服务器状态查询</title>
	<script type="text/javascript" src="http://libs.baidu.com/jquery/1.7.2/jquery.min.js"></script>
	<style>
		*{
			font-size:14px;
			margin:0px;
			padding:0px;
		}
		.inputtype{
			width:85px;
		}
		/* 容器 */
		#container{
			width:900px;
			height:auto;
			margin:1px auto 0;
		}
		/* 顶部 */
		#top{
			width:880px;
			height:35px;
			line-height:35px;
			margin:1px auto 0;
			padding-left:20px;
			color:white;
			background:gray;
			border-radius:3px;
		}
		/* 主要功能 */
		#main{
			margin-top:2px;
			margin-bottom:2px;
			width:900px;
		}
		/* 左边部分[系统运行状态] */
		#main_left{
			width:600px;
			float:left;
		}
		/* 缓存命中率 */
		#main_left_cached{
			width:600px;
			height:290px;
		}	
		/* 右边部分[系统运行变量] width:300px */
		#main_right{
			width:297px;
			float:right;
			border-left:1px solid white;
			padding-left:2px;
			overflow:hidden;
		}
		#main_right ul{
			list-style:none;
		}
		#main_right ul li{
			height:25px;
			line-height:25px;
			border-bottom:1px solid #DFDFDF;
			overflow:hidden;
		}

	</style>
<script>
//保存mysql运行变量中文译名
var mysql_variable ={ 
	auto_increment_increment:'自增设置',
	auto_increment_offset:'自动偏移',
	autocommit:'自动提交[事务]',
	automatic_sp_privileges:'自动SP特权',
	back_log:'可能连接数量',
	basedir:'MySQL安装路径',
	big_tables:'大表',
	binlog_cache_size:'二进制日志缓冲大小',
	binlog_format:'二进制日志缓存格式',
	bulk_insert_buffer_size:'插入缓存区大小',
	character_set_client:'客服端字符集',
	character_set_connection:'服务器字符集',
	character_set_database:'服务器数据库字符集',
	character_set_filesystem:'服务器文件系统字符集',
	character_set_server:'服务器字符集',
	character_set_system:'服务器系统字符集',
	character_sets_dir:'服务器字符集地址',
	collation_connection:'字符集比较',
	connect_timeout:'连接超时',
	datadir:'数据保存地址',
	date_format:'日期格式',
	datetime_format:'时间格式',
	default_week_format:'默认周格式',
	delay_key_write:'延迟写入',
	delayed_insert_limit:'延迟插入',
	delayed_insert_timeout:'插入延迟超时',
	delayed_queue_size:'延迟队列大小',
	div_precision_increment:'除法精度',
	engine_condition_pushdown:'引擎条件下推',
	error_count:'错误次数',
	event_scheduler:'事务调度是否支持',

};
//保存mysql运行状态中文译名
var mysql_status = {
	com_alter_db:'更改数据库',
	com_check : '检查命令',


};
$(document).ready(function(){
//页面载入后运行开始
//共用
var sql = {};//建立sql对象

//提交框信息
function GetInput(){
	var i = {
	local:$("input[name='local']").val(),
	port:$("input[name='port']").val(),
	user:$("input[name='user']").val(),
	pwd:$("input[name='pwd']").val(),
	time:$("input[name='time']").val(),
	}
	return i;
};var info = GetInput();//数据信息
var UpdataTime = info['time']*1000;//更新时间
//console.log(UpdataTime);

/* @func 获取文件名 */
function GetFileName(){
	var url = location.href;
	var file = url.split('?');
	var pos = file[0].lastIndexOf('/');
	//console.log(file[0].substr(pos+1));
	return file[0].substr(pos+1);
}var gfn = GetFileName();

/* @func 操作提示 | 1s */
function N(n,color){
	$('#pack_up_operation').html(n).
		fadeIn(300,function(){
			$(this).fadeOut(700);
		}).css('color', typeof color ? 'blue' : color);
}
///Notice('haoasdasdasdfasd');

/* 共用提交 */
sql.common = function(locals,ports,users,pwds,signs,callback){
//	console.log(arguments[4]);
	$.post(gfn,{
		localhost:locals,
		port: ports,
    	user: users,
		pwd : pwds,
		type : signs,
	},function(data){
		//console.log(data);
		$('#sql_global_variables').data(signs,eval('('+data+')'));
		if(typeof callback =='function'){
			callback(eval('('+data+')'));
		}
	});
}
//sql.common();


/*获取mysql运行变量*/
function variable(callback,t){
	$.post(gfn,{
		'localhost':'localhost',
		'port':'3306',
    	'user':'root',
		'pwd' :'',
		'type':'variable'
	},function(data){
		$('#sql_global_variables').data('variable',eval('('+data+')'));
		if(typeof callback == 'function'){
			callback(eval('('+data+')'));
		}
	});
}
/* 获取mysql运行变量2 */
function variable2(callback){
	sql.common(info['local'],info['port'],info['user'],info['pwd'],'variable',callback);
}


/*获取mysql运行状态*/
function status(callback){
	$.post(gfn,{
		'localhost':'localhost',
		'port':'3306',
		'user':'root',
		'pwd':'',
		'type':'status'
	},function(data){
		if(typeof callback == 'function'){
			callback(eval('('+data+')'));
		}
	});
}
/* 获取mysql运行状态2 */
function status2(callback){
	sql.common(info['local'],info['port'],info['user'],info['pwd'],'status',callback);
}

//对变量进行处理
function var_del(){
	variable2(function(data){
		//var $('#main_right ul li').html());
		var tpl = '<li><a title="test">test</a></li>';
		//$('#main_right').append($(tpl).html('123').attr('title','123'));
		for(name in data){
			var chname = mysql_variable[name];
			if(chname){
				var content = name+':'+data[name];
				$('#main_right ul').append($(tpl).
								   html(content).
								   attr('title', mysql_variable[name]+':'+data[name]));
			}else{
				var content = name+':'+data[name];
				$('#main_right ul').append($(tpl).
								   html(content).
					 			   attr('title',content));
			}
		}
	});
}
var_del();


//信息提交触动
$('input[name=run]').click(function(){
	setTimeout(function(){	
		N('正在提交信息...');
		clearInterval(timer);//消除定时器
		setTimeout(function(){
			N('两秒后重新开始');
			setTimeout(function(){
				info = GetInput();//重新获取提交信息
				UpdataTime = info['time']*1000;//更新时间
				G();//重新执行
				},2000);
		},2000);
	}, 1000);	
});

G();//执行一次

var timer;//定时器标示

function G(){//G方法开始
//定时执行MySQL服务健康状态分析
timer = setInterval(function(){
//对运行状态处理
status2(function(data){

//检查错误
//console.log(typeof data['error']);
if(typeof data['error'] == 'string'){
	N(data['error']);
}
	
var variables = $('#sql_global_variables').data('variable');

//console.log(info);

//查询缓存命中率
function cached_query(data){
	var sum = data['qcache_hits']+data['qcache_inserts'];
	var ncached = data['qcache_hits'];
	if(ncached || sum ){
		return 1;
	}else{
		return 1-(ncached/sum);
	}
}
/*查询缓存命中率显示在页面上*/
function cached_query_show(data){
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = cached_query(data);

	var color = '';
	for(var i=0;i<6;i++){
		var sj = Math.random();
		color += ((sj*10).toString()).substr(0,1);
	}
	//console.log(color);
	//VAR R = 'css('background','#'+rand)';
	var content = $(tpl).append($(tpl_one).css('height',h*200).css('background','#'+color).attr('title',h*100+'%命中'));	
	//console.log(content);
	$('#cached_query').append(content);

	//子元素总数
	var cd = $('#cached_query').children('.cached_query_dph');
	if(cd.length > 60){
		$('#cached_query').children('.cached_query_dph').first().remove();
	}
}
cached_query_show(data);




//关键字效率[缓存率]
function key_effic(data){
	return 1-(data['key_reads']/data['key_read_requests']);
}
/* key值缓存率显示在页面上 */
function key_effic_show(data){
	var keyeffic = key_effic(data);
	//console.log(keyeffic);
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = keyeffic;
	var color = '';
	for(var i=0;i<6;i++){var sj = Math.random();color += ((sj*10).toString()).substr(0,1);}
	var content = $(tpl).append($(tpl_one).css('height',h*200).css('background','#'+color).attr('title',h*100+'%命中'));	
	$('#cached_key').append(content);

	//子元素总数
	var cd = $('#cached_key').children('.cached_query_dph');
	if(cd.length > 60){
		//console.log(cd.length);
		$('#cached_key').children('.cached_query_dph').first().remove();
	}

}
key_effic_show(data);


//线程缓存命中率
function thread_effic(data){
	//console.log(data['connections'],data['threads_created']);
	return 1-data['threads_created']/data['connections'];
}
/* key值缓存率显示在页面上 */
function thread_effic_show(data){
	var threadeffic = thread_effic(data);
	//console.log(threadeffic);
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = threadeffic;
	var color = '';
	for(var i=0;i<6;i++){var sj = Math.random();color += ((sj*10).toString()).substr(0,1);}
	var content = $(tpl).append($(tpl_one).css('height',h*200).css('background','#'+color).attr('title',h*100+'%命中'));	
	$('#cached_thread').append(content);
	//子元素总数
	var cd = $('#cached_thread').children('.cached_query_dph');
	if(cd.length > 60){
		$('#cached_thread').children('.cached_query_dph').first().remove();
	}
}
thread_effic_show(data);

//innodb缓存池命中率
function innodb_effic(data){
	//console.log(data);
	//console.log(data['innodb_buffer_pool_read_requests'],data['innodb_buffer_pool_read_requests'],data['innodb_buffer_pool_read_ahead'],data['innodb_buffer_pool_reads']);
	return 1-data['innodb_buffer_pool_read_requests']/(data['innodb_buffer_pool_read_requests']+
		data['innodb_buffer_pool_read_ahead_rnd']+data['innodb_buffer_pool_read_ahead_seq']
		+data['innodb_buffer_pool_reads']);
}
//innodb缓存池命中率显示在页面上
function innodb_effic_show(data){
	
	var innodbeffic = innodb_effic(data);
	//console.log(innodbeffic);
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = innodbeffic;
	var color = '';
	for(var i=0;i<6;i++){var sj = Math.random();color += ((sj*10).toString()).substr(0,1);}
	var content = $(tpl).append($(tpl_one).css('height',h*200).css('background','#'+color).attr('title',h*100+'%命中'));	
	$('#cached_innode_pool').append(content);
	//子元素总数
	var cd = $('#cached_innode_pool').children('.cached_query_dph');
	if(cd.length > 60){
		$('#cached_innode_pool').children('.cached_query_dph').first().remove();
	}
}
innodb_effic_show(data);


//table扫描比
function tablescan_effic(data){
	return 1-(data['handler_read_rnd_next']+data['handler_read_rnd'])/(data['handler_read_rnd_next']+data['handler_read_rnd']+data['handler_read_key']+data['handler_read_prev']);
}
function tablescan_effic_show(data){	
	var tablescan = tablescan_effic(data);
	//console.log(threadeffic);
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = tablescan;
	var color = '';
	for(var i=0;i<6;i++){var sj = Math.random();color += ((sj*10).toString()).substr(0,1);}
	var content = $(tpl).append($(tpl_one).css('height',h*200).css('background','#'+color).attr('title',h*100+'%命中'));	
	$('#cached_tablesan').append(content);
	//子元素总数
	var cd = $('#cached_tablesan').children('.cached_query_dph');
	if(cd.length > 60){
		$('#cached_tablesan').children('.cached_query_dph').first().remove();
	}
}
tablescan_effic_show(data);





//连接使用率
function connection_usage(data,variables){
	return (data['threads_connected']/variables['max_connections']);
}
/* 展示在页面上 */
function connection_usage_show(data,variables){
	var con_use =  connection_usage(data,variables);//连接使用率
	//console.log(con_use*500);
	var tpl = '<div class="connection_use" style="height:13px;float:left;"></div>';
	var color = '';
	for(var i=0;i<6;i++){var sj = Math.random();color += ((sj*10).toString()).substr(0,1);}

	var test = $(tpl).css('width',con_use*500).css('background','#'+color).attr('title','正在连接的:'+data['threads_connected']+'最大连接:'+variables['max_connections']+'连接使用率:'+con_use);
	//console.log(test);
	$('#other_connection_use .connection_use').replaceWith(test);
}
connection_usage_show(data,variables);


//接受数据/发送的数据
function rev_send_data(data){
	return [data['bytes_received']/(1024*1024),data['bytes_sent']/(1024*1024)];
}
//展示在页面上
function rev_send_data_show(data){
	var rsd = rev_send_data(data);
	var tpl = '<div class="red_use" style="width:300px;float:left;font-size:12px;"></div>';
	//console.log($(tpl).html('接受的数据:'+rsd[0]));
	$('#other_rev_send').html('');
	$('#other_rev_send').append($(tpl).html('接受的数据:'+rsd[0] + 'M')).
						 append($(tpl).html('发送的数据:'+rsd[1] + 'M'));
}
rev_send_data_show(data);



//运行时间显示在页面上
function uptime_show(data){
	var tpl = '<div class="uptime" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;"></div>';
	var times = new Date();
	s = times.getTime();
	//console.log(times.getTime());
	//console.log('结束时间:'+ s +"运行时间:"+ data['uptime']);
	var btime = s - data['uptime']*1000;//开始运行时间
	//console.log('开始运行:'+btime);	
	function gett(btime){
		//console.log(btime);
		var time = new Date();
		time.setTime(btime);
		//开始时间年月日 时:分:秒
		var get_btime = '';
		get_btime += time.getFullYear() + '年';
		get_btime += time.getMonth() + 1 + '月';
		get_btime += time.getDate() + '日 ';
		get_btime += time.getHours() + ':';
		get_btime += time.getMinutes() + ':';
		return get_btime += time.getSeconds();
	}
	var get_btime = gett(btime);
	$('#other_uptime').html($(tpl).html('开始运行:'+get_btime+'运行时间:'+Math.floor(data['uptime']/60) +'分钟'+data['uptime']%60+'秒'));
}
uptime_show(data);




//获取各种查询次数
function sql_times(data){
	var sql = {};
	for( com in data ){
		if(com.substr(0,4)=='com_'){
			sql[com] = data[com];
		}
	}
	return sql;
}
//各种命令查询次数显示在页面上
function sql_query_times(data){
	$('#main_left_com').html('');
 	var o = sql_times(data);
	var tpl = '<div class="com" style="height:20px;width:300px;float:left;line-height:20px;"></div>';
	for(a in o){
		var chname = mysql_status[a];
		if(chname){
			$('#main_left_com').append($(tpl).html(a+':'+o[a]).attr('title', chname +':'+o[a]));
		}else{
			$('#main_left_com').append($(tpl).html(a+':'+o[a]).attr('title', a +':'+o[a]));
		}
	}
}
sql_query_times(data);


//console.log(location.href);


});
//定时结束
},UpdataTime);
//G方法结束
}
//页面载入后运行代码结束
})
</script>	
</head>
<body>
<!-- 保存服务全局变量 -->
<div id="sql_global_variables" style="height:0;overflow:hidden;">test</div>
<!-- 操作提示 -->
<div id="pack_up_operation" style="
position:fixed;top:0px;right:0px;height:20px;width:220px;overflow:hidden;"></div>
	<!-- big container -->
	<div id="container">
		<!-- 顶部 -->
		<div id="top">
			<form action="" method="POST" name="">
				<span>服务器地址:<input class="inputtype" type="text" name="local" value="127.0.0.1" /></span>
				<span>端口:<input class="inputtype" type="text" name="port" value="3306" /></span>
				<span>用户名:<input class="inputtype" type="text" name="user" value="root" /></span>
				<span>密码:<input class="inputtype" type="password" name="pwd" value="root" /></span>
				<span>间隔刷新时间:<input class="inputtype" type="text" name="time" value="3" />秒</span>
				<span><input type="button" name="run" value="执行" style="height:20px;width:50px;" /></span>
			</form>
		</div>
			
		<!-- 主要功能区 -->
		<div id="main">
			
			<!-- 左边为运行状态监测 -->
			<div id="main_left">
				<!-- 缓存命中率 -->
				<div id="main_left_cached" style="overflow:hidden;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">(Query)查询缓存命中率</h3>
					<!-- 运行状态图 -->
					<div id="cached_query" style="height:200px;">
						<!--<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;">
<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div></div>-->
								
					</div>
					<!-- 一些建议 -->
					<div id="cached_query_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>

				<!-- 键值命中率 -->
				<div id="main_left_key" style="overflow:hidden;margin-top:5px;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">(KEY)键值缓存命中率</h3>
					<!-- 运行状态图 -->
					<div id="cached_key" style="height:200px;"></div>
					<!-- 一些建议 -->
					<div id="cached_key_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>

				<!-- 线程命中率 -->
				<div id="main_left_thread" style="overflow:hidden;margin-top:5px;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">(Threads)键值缓存命中率</h3>
					<!-- 运行状态图 -->
					<div id="cached_thread" style="height:200px;"></div>
					<!-- 一些建议 -->
					<div id="cached_thread_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>

				<!-- innodb缓冲池命中率 -->
				<div id="main_left_innodb_pool" style="overflow:hidden;margin-top:5px;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">(InnoDB)缓冲池命中率</h3>
					<!-- 运行状态图 -->
					<div id="cached_innode_pool" style="height:200px;"></div>
					<!-- 一些建议 -->
					<div id="cached_innodb_pool_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>

				<!-- 全表扫描比 -->
				<div id="main_left_tablescan" style="overflow:hidden;margin-top:5px;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">(Tbale)全表扫描比</h3>
					<!-- 运行状态图 -->
					<div id="cached_tablesan" style="height:200px;"></div>
					<!-- 一些建议 -->
					<div id="cached_tablesan_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>

				
				

				<!-- other status -->
				<div id="main_left_other" style="overflow:hidden;margin-top:5px;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">其他的一些属性</h3>
					<!-- 运行状态图 -->
					<div id="other" style="">
						<!-- 连接使用率 -->
						<div id='other_connection_use' style="width:600px;height:15px;margin-top:3px;"><span style="float:left;font-size:12px;">连接使用率:</span><div class="connection_use"></div></div>
						<!-- 接受和发送信息 -->
						<div id='other_rev_send' style="width:600px;height:15px;margin-top:3px;"></div>
						<!-- 运行时间 -->
						<div id='other_uptime' style="width:600px;height:15px;margin-top:2px;margin-bottom:2px;"></div>

					</div>
					<!-- 一些建议 -->
					<div id="cached_other_advise" style="height:60px;text-align:center;background:#ccd;"></div>
				</div>
				
				<!-- 各种命令使用次数 -->
				<div id="main_left_com" style="overflow:hidden;margin-top:10px;">
					<div class="com" style="height:20px;width:300px;float:left;line-height:20px;">con_insert:ssdo</div>
				</div>




			</div>
			<!-- 右边为mysql系统变量 -->
			<div id='main_right'>
				<ul></ul>
			</div>

			
		</div>


		<div style="clear:both;"></div>
		<div style="width:900px;height:35px;"></div>

		<!-- 底部 -->
		<div style="clear:both;"></div>
		<div id="midoks" style="font-size:14px;background:gray;width:900px;height:35px;position:fixed;bottom:0;">
			<span style="display:block;width:900px;height:35px;line-height:35px;margin-left:auto;margin-right:auto;text-align:center;color:white;">
				编写:midoks(midoks.cachecha.com)
			</span>
		</div>
	<div>
</body>
</html>

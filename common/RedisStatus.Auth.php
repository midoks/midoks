<?php
/**
 *	MemCache服务器的健康状态 v1.0
 *	@author:midoks
 */

define('SYS_USER', 'midoks');
define('SYS_USER_PWD', md5('cjscjs123'));


$mem_acl = new RedisAcl();
$mem_acl->acl();


if(isset($_POST['submit'])){
	//echo json_encode($_POST);exit;
	$m = new RedisStuts();
	$m->start();
}
class RedisStuts{

	public $linkID = null; 

	public function __construct(){
		$this->linkID = new Redis();
		
		if(isset($_POST['local']) && isset($_POST['port'])){
			$this->linkID->connect($_POST['local'], $_POST['port']);
			if(!is_object($this->linkID)){
				echo "{error:'连接错误'}";
			}
		}else{
			$this->linkID->connect('127.0.0.1', 6379);
		}
		if(!is_object($this->linkID)){
			echo "{error:'连接错误'}";
		}
	}

	public function start(){
		//防止nginx登陆,直接跳入
		if(!isset($_POST['user'])){
			echo json_encode($this->linkID->info());exit;
		}
	}

	public function __destruct(){
		if(!$this->linkID){
			$this->linkID->close();
		}
	}

}


//权限设置
class RedisAcl{

	//权限管理
	public function acl(){
		if ( PHP_SAPI == 'apache2handler'){
			$this->_acl_apache();
		} else if (strpos(PHP_SAPI, 'fpm') !== false ){
			$this->_acl_nginx();
		} else {
			exit('need nginx or apache as server');
		}
	}

	/**
	 * 通过apache判断权限
	 */
	private function _acl_apache(){
		$user = isset($_SERVER['PHP_AUTH_USER']) ? $_SERVER['PHP_AUTH_USER'] : '';
		$pass = isset($_SERVER['PHP_AUTH_PW']) ? $_SERVER['PHP_AUTH_PW'] : '';
		$type = isset($_SERVER['AUTH_TYPE']) ? $_SERVER['AUTH_TYPE'] : '';

		if(!empty($user) && !empty($pass)){
			if ($this->_check_user($user, $pass)){
			} else {
				header("WWW-Authenticate:Basic realm='Private'");
				header('HTTP/1.0 401 Unauthorized');
    			print "You are unauthorized to enter this area.";
    			exit;
			}
		} else {
			header("WWW-Authenticate:Basic realm='Private'");
    		header('HTTP/1.0 401 Unauthorized');
    		print "You are unauthorized to enter this area.";
    		exit;
		}
	}

	/**
	 * 通过nginx判断权限
	 */
	private function _acl_nginx(){
		session_start();

		if(isset($_POST)){
			$user = isset($_POST['user']) ? $_POST['user'] : '';
			$pwd = isset($_POST['pwd']) ? $_POST['pwd'] : '';

			if(!empty($user) && !empty($pwd)){
				if ($this->_check_user($user, $pwd)){

					$_SESSION['isLogin'] = array(
						'user' => $user,
						'pwd'  => $pwd,
					);
				} else {
					$_SESSION['isLogin'] = NULL;
					$this->login_err = "登陆失败";
				}
			}
		}

		if ( isset($_SESSION['isLogin']) && $_SESSION['isLogin']) {
			$user = isset($_SESSION['isLogin']['user']) ? $_SESSION['isLogin']['user'] : '';
			$pwd = isset($_SESSION['isLogin']['pwd']) ? $_SESSION['isLogin']['pwd'] : '';
			$this->_check_user($user, $pwd);
		} else {
			$this->ngx_login();
			exit();
		}
	}

	private function _check_user($user, $passwod){
		if(SYS_USER == $user && SYS_USER_PWD == md5($passwod)){
			return true;
		}
		return false;
	}

	private function ngx_login(){
$login_page = <<<EOF
<style type="text/css">
.login{
	width: 300px;
	height: 200px;
	margin: 100px auto auto auto;
	/*background-color: red;*/
	border: 1px solid gray;
}	
</style>
<form id='main_upload' method='POST'>
<div class='login'>

	<div  style="width:200px;margin: 70px auto auto auto;">
		<table>
		<tr>
		<td style="text-align:right;">用户名:</td>
		<td><input type="text" name="user" value="" /> </td>
		<tr>
		<tr>
		<td style="text-align:right;">密码:</td>
		<td><input type="password" name="pwd" value="" /> </td>
		<tr>
		<tr>
		<td colspan="2" style="text-align:center;">
			<input type="submit" name="submit" value="提交">
		</td>
		</tr>
		</table>
	</div>
</div>
</form>
EOF;
		echo($login_page);
	}
}


?>
<!DOCTYPE html>
<html>
<head>
	<title>RedisCache服务器运行状态online</title>
	<meta http-equiv="Content-Type" content="text/html;charset=utf-8"/>
<script type="text/javascript" src="http://libs.baidu.com/jquery/1.7.2/jquery.min.js"></script>
<style>
*{font-size:14px;margin:0px;padding:0px;}
#m_right ul{list-style:none;}
#m_right ul li{height:25px;line-height:25px;border-bottom:1px solid #DFDFDF;overflow:hidden;}
</style>
<script>
//http://redisdoc.com/server/info.html
var redis_var = {
	//server 部分记录了 Redis 服务器的信息
	redis_version:'Redis服务器版本',
	redis_git_sha1:'Git SHA1',
	redis_git_dirty:'Git dirty flag',
	os:'Redis服务器的宿主操作系统',
	arch_bits:'架构(32或64位)',
	multiplexing_api:'Redis所使用的事件处理机制',
	gcc_version:'编译Redis时所使用的GCC版本',
	process_id:'服务器进程的PID',
	run_id:'Redis服务器的随机标识符(用于Sentinel和集群)',
	tcp_port:'TCP/IP监听端口',
	uptime_in_seconds:'自Redis服务器启动以来,经过的秒数',
	uptime_in_days:'自Redis服务器启动以来,经过的天数',
	lru_clock:'以分钟为单位进行自增的时钟,用于LRU管理',
	//clients 部分记录了已连接客户端的信息
	connected_clients:'已连接客户端的数量(不包括通过从属服务器连接的客户端)',
	client_longest_output_list:'当前连接的客户端当中,最长的输出列表',
	client_longest_input_buf:'当前连接的客户端当中,最大输入缓存',
	blocked_clients:'正在等待阻塞命令(BLPOP|BRPOP|BRPOP|LPUSH)的客户端的数量',
	//memory 部分记录了服务器的内存信息
	used_memory:'由Redis分配器分配的内存总量,以字节(byte)为单位',
	used_memory_human:'以人类可读的格式返回Redis分配的内存总量',
	used_memory_rss:'从操作系统的角度,返回Redis已分配的内存总量(俗称常驻集大小)这个值和top,ps等命令的输出一致',
	used_memory_peak:'Redis的内存消耗峰值(以字节为单位)',
	used_memory_peak_human:'以人类可读的格式返回Redis的内存消耗峰值',
	used_memory_lua:'Lua引擎所使用的内存大小(以字节为单位)',
	mem_fragmentation_ratio:'used_memory_rss和used_memory之间的比率',
	mem_allocator:'在编译时指定的,Redis所使用的内存分配器.可以是libc,jemalloc或者tcmalloc',
	//persistence部分记录了跟RDB持久化和AOF持久化有关的信息
	loading:'一个标志值,记录了服务器是否正在载入持久化文件',
	rdb_changes_since_last_save:'距离最近一次成功创建持久化文件之后,经过了多少秒',
	rdb_bgsave_in_progress:'一个标志值,记录了服务器是否正在创建RDB文件',
	rdb_last_save_time:'最近一次成功创建RDB文件的UNIX时间戳',
	rdb_last_bgsave_status:'一个标志值,记录了最近一次创建RDB文件的结果是成功还是失败',
	rdb_last_bgsave_time_sec:'记录了最近一次创建 RDB 文件耗费的秒数',
	rdb_current_bgsave_time_sec:'如果服务器正在创建RDB文件,那么这个域记录的就是当前的创建操作已经耗费的秒数',
	aof_enabled:'一个标志值,记录了AOF是否处于打开状态',
	aof_rewrite_in_progress:'一个标志值,记录了服务器是否正在创建AOF文件',
	aof_rewrite_scheduled:'一个标志值,记录了在RDB文件创建完毕之后,是否需要执行预约的AOF重写操作',
	aof_last_rewrite_time_sec:'最近一次创建 AOF 文件耗费的时长',
	aof_current_rewrite_time_sec:'如果服务器正在创建AOF文件,那么这个域记录的就是当前的创建操作已经耗费的秒数',
	aof_last_bgrewrite_status:'一个标志值,记录了最近一次创建AOF文件的结果是成功还是失败',
		//如果 AOF 持久化功能处于开启状态，那么这个部分还会加上以下域
	aof_current_size:'AOF文件目前的大小',
	aof_base_size:'服务器启动时或者AOF重写最近一次执行之后,AOF文件的大小',
	aof_buffer_length:'AOF缓冲区的大小',
	aof_rewrite_buffer_length:'AOF 重写缓冲区的大小',
	aof_pending_bio_fsync:'后台I/O队列里面,等待执行的fsync调用数量',
	aof_delayed_fsync:'被延迟的fsync调用数量',

	//replication:主/从复制信息
	role:'如果当前服务器没有在复制任何其他服务器,那么这个域的值就是master;否则的话,这个域的值就是 slave注意,在创建复制链的时候,一个从服务器也可能是另一个服务器的主服务器',
		//如果当前服务器是一个从服务器的话，那么这个部分还会加上以下
	master_host:'主服务器的IP地址',
	master_port:'主服务器的TCP监听端口号',
	master_link_status:'复制连接当前的状态,up表示连接正常,down表示连接断开',
	master_last_io_seconds_ago:'距离最近一次与主服务器进行通信已经过去了多少秒钟',
	master_sync_in_progress:'一个标志值,记录了主服务器是否正在与这个从服务器进行同步',
		//如果同步操作正在进行，那么这个部分还会加上以下
	master_sync_left_bytes:'距离同步完成还缺少多少字节数据',
	master_sync_last_io_seconds_ago:'距离最近一次因为SYNC操作而进行I/O已经过去了多少秒',
		//如果主从服务器之间的连接处于断线状态，那么这个部分还会加上以下
	master_link_down_since_seconds:'主从服务器连接断开了多少秒',
	connected_slaves:'已连接的从服务器数量',
	//cpu部分记录了 CPU 的计算量统计信息
	used_cpu_sys:'Redis服务器耗费的系统CPU',
	used_cpu_user:'Redis服务器耗费的用户CPU',
	used_cpu_sys_children:'后台进程耗费的系统CPU',
	used_cpu_user_children:'后台进程耗费的用户CPU',

	cluster_enabled:'一个标志值,记录集群功能是否已经开启',

	//其他
	total_connections_received:'运行以来连接过的客户端的总数量',
	total_commands_processed:'运行以来执行过的命令的总数量',
	instantaneous_ops_per_sec:'服务器每秒钟执行的命令数量',
	rejected_connections:'因为最大客户端数量限制而被拒绝的连接请求数量',
	expired_keys:'运行以来过期的key的数量',
	evicted_keys:'运行以来删除过的key的数量',
	keyspace_hits:'命中key的次数',
	keyspace_misses:'不命中key的次数',
	pubsub_channels:'当前使用中的频道数量',
	pubsub_patterns:'当前使用的模式的数量',


};
$(document).ready(function(){
//begin
function GetFileName(){
	var url = location.href;
	var file = url.split('?');
	var pos = file[0].lastIndexOf('/');
	return file[0].substr(pos+1);
}var gfn = GetFileName();

//提交框信息
function GetInput(){
	var i = {
	local:$("input[name='local']").val(),
	port:$("input[name='port']").val(),
	time:$("input[name='time']").val(),
	}
	return i;
};var info = GetInput();//数据信息
var UpdataTime = info['time']*1000;//更新时间

function N(n,color){
	$('#pack_up_operation').html(n).
		fadeIn(300,function(){
			$(this).fadeOut(700);
		}).css('color', typeof color ? 'blue' : color);
}

function get_var(callback){
	$.post(gfn,{
		local:$("input[name='local']").val(),
		port:$("input[name='port']").val(),
		submit:'true',
	},function(data){
		if(typeof callback == 'function'){
			callback(eval('('+data+')'));
		}
	});
}

//列表显示
function list_var(data){
	var m_right = $('#m_right ul');
	var tpl = '<li><a title="test">test</a></li>';
	$(m_right).html('');
	//console.log(m_right);
	for(i in data){
		var ri = redis_var[i];
		var content = i+':'+data[i];
		//console.log(i, ":",data[i]);
		if(ri){
			$(m_right).append($(tpl).html(content).attr('title', ri+':'+data[i]));
		}else{
			$(m_right).append($(tpl).html(content).attr('title', content));
		}
	}
}


function cached_mzl(data){
	//console.log(data);
	if(0 == data['keyspace_hits']){
		return 1;
	}
	var h = 1 - (data['keyspace_misses']/data['keyspace_hits']);
	return h;
}


function cached_query_mzl(data){
	var tpl_one = '<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;"></div>';
	var tpl = '<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;"></div>';
	var h = cached_mzl(data);

	var color = '';
	for(var i=0;i<6;i++){
		var sj = Math.random();
		color += ((sj*10).toString()).substr(0,1);
	}

	var content = $(tpl).append($(tpl_one).
		css('height',h*200).css('background','#'+color).
		attr('title',h*100+'%命中'));

	$('#m_left_cached').append(content);

	//子元素总数
	var cd = $('#m_left_cached').children('.cached_query_dph');
	if(cd.length > 60){
		$('#m_left_cached').children('.cached_query_dph').first().remove();
	}
}

function redis_attr(data){
	$('#m_left_attr').html('');
	var h_tpl = '<h4 style="margin-top:5px;font-size:22px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">其他的一些属性</h4>';

	var div_tpl = '<div id="memcache_attr_list"></div>';
	
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
	var get_btime = gett(data['uptime_in_seconds']);
	var p = $(div_tpl).append('<div class="uptime" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">运行时间:'+ Math.floor((data['uptime_in_seconds']/60)/60) + '小时' + Math.floor((data['uptime_in_seconds']/60)%60)+'分钟'+data['uptime_in_seconds']%60+'秒'+'</div>');
	
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">使用内存:'+ data['used_memory_human']+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">端口:'+data['tcp_port']+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">服务器每秒钟执行的命令数量:'+data['instantaneous_ops_per_sec']+'次'+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">因为最大客户端数量限制而被拒绝的连接请求数量:'+data['rejected_connections']+'次'+'</div>');


	
	//end in below
	$('#m_left_attr').append(h_tpl);
	$('#m_left_attr').append(p);
}

function all_var(){
	get_var(function(data){
		//列表显示
		list_var(data);
		cached_query_mzl(data);

		redis_attr(data);
	});
}
redis_all();
function redis_all(){
	all_var();
}

var timer;
function timer_start_and_time(s){
	//console.log(s);
	if('stop'==s){
		clearInterval(timer);
	}else{
		timer = setInterval(function(){
			redis_all();
		}, s);
	}
}

timer_start_and_time(UpdataTime);
$('input[name=run]').click(function(){
	setTimeout(function(){
		N('正在提交信息...','red');
		timer_start_and_time('stop');
		setTimeout(function(){
			N('两秒后重新开始');
			setTimeout(function(){
				info = GetInput();//重新获取提交信息
				UpdataTime = info['time']*1000;//更新时间
				timer_start_and_time(UpdataTime);
				},1000);
		},1000);
	}, 1000);	
});

//end
});//
</script>
</head>
<body>
	<!-- 提示 -->
	<div id="pack_up_operation" style="position:fixed;top:0px;right:0px;height:20px;width:220px;overflow:hidden;"></div>

	<div id="container" style="width:900px;height:auto;margin:1px auto 0;">
		<div id="top" style="width:880px;height:35px;line-height:35px;margin:1px auto 0;padding-left:20px;color:white;background:gray;border-radius:3px;">
			<form action=""method="POST" name="">
				<span>服务器地址:<input class="inputtype" type="text" name="local" value="127.0.0.1" /></span>
				<span>端口:<input class="inputtype" type="text" name="port" value="6379" /></span>
				<span>间隔刷新时间:<input class="inputtype" type="text" name="time" value="2" />秒</span>
				<span><input type="button" name="run" value="执行" style="height:20px;width:50px;" /></span>
			</form>
		</div>
		
		<div id="main">
			<div id="m_left" style="float:left;width:600px;">
				<div id="m_left_cached" style="overflow:hidden;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">
						RedisCache命中率
					</h3>
					<!--<div class="cached_query_dph" style="position:relative;float:left;height:200px;width:10px;border-radius:3px;">
					<div class="cached_query_dph_one" style="position:absolute;bottom:0;left:2px;width:6px;height:10px;background:blue;">
					</div></div>-->
				</div>

				<div id="m_left_attr"></div>
				
			</div>


			<div id="m_right" style="float:right;width:297px;">
				<ul>
					<!--<li>爱的</li>-->
				</ul>
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
	</div>
<body>
</html>

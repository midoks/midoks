<?php
/**
 *	MemCache服务器的健康状态 v1.0
 *	@author:midoks
 */

define('MEM_USER', 'midoks');
define('MEM_USER_PWD', md5('cjscjs123'));


$mem_acl = new MemAcl();
$mem_acl->acl();


if(isset($_POST['submit'])){
	//echo json_encode($_POST);exit;
	$m = new MemCacheStuts();
	$m->start();
}
class MemCacheStuts{

	public $linkID = null; 

	public function __construct(){
		$this->linkID = new Memcache();
		
		if(isset($_POST['local']) && isset($_POST['port'])){
			$this->linkID->connect($_POST['local'], $_POST['port']);
			if(!is_object($this->linkID)){
				echo "{error:'连接错误'}";
			}
		}else{
			$this->linkID->connect('localhost', 11211);
		}
		if(!is_object($this->linkID)){
			echo "{error:'连接错误'}";
		}
	}

	public function start(){
		//防止nginx登陆,直接跳入
		if(!isset($_POST['user'])){
			echo json_encode($this->linkID->getStats());exit;
		}
	}

	public function __destruct(){
		if(!$this->linkID){
			$this->linkID->close();
		}
	}

}


//权限设置
class MemAcl{

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
		if(MEM_USER == $user && MEM_USER_PWD == md5($passwod)){
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
	<title>MemCache服务器运行状态online</title>
	<meta http-equiv="Content-Type" content="text/html;charset=utf-8"/>
<script type="text/javascript" src="http://libs.baidu.com/jquery/1.7.2/jquery.min.js"></script>
<style>
*{font-size:14px;margin:0px;padding:0px;}
#m_right ul{list-style:none;}
#m_right ul li{height:25px;line-height:25px;border-bottom:1px solid #DFDFDF;overflow:hidden;}
</style>
<script>
var memcache_var = {
	pid:'运行ID',
	uptime:'运行时间',
	time:'当前时间戳',
	version:'版本',
	pointer_size:'当前操作系统的指针大小',
	curr_items:'服务器当前存储的items数量',
	total_items:'从服务器启动以后存储的items总数量',
	bytes:'当前服务器存储items占用的字节数',
	curr_connections:'当前打开着的连接数',
	total_connections:'从服务器启动以后曾经打开过的连接数',
	connection_structures:'服务器分配的连接构造数',
	cmd_get:'get命令(获取)总请求次数',
	cmd_set:'set命令(保存)总请求次数',
	get_hits:'总命中次数',
	get_misses:'总未命中次数',
	evictions:'为获取空闲内存而删除的items数',
	bytes_read:'总读取字节数(请求字节数)',
	bytes_written:'总发送字节数(结果字节数)',
	limit_maxbytes:'分配给memcache的内存大小(字节)',
	threads:'当前线程数',
	cmd_flush:'指向flush_all命令总数',
	delete_misses:'delete未命中次数',
	delete_hits:'delete命中次数',  
	incr_misses:'incr未命中次数',
	incr_hitsincr:'命中次数',
	decr_hits:'decr命中次数',
	decr_misses:'decr未命中次数',
    cas_misses: 'cas未命中次数',
    cas_hits:  'cas命中次数',
    cas_badval: '使用擦拭次数',
    auth_cmds:'认证失败次数',                                         
    auth_errors:'认证错误次数',                                         
    accepting_conns:'目前接受的链接数',
    incr_hits:'incr命中次数',                                       

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
		var ri = memcache_var[i];
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
	if(0 == data['cmd_get']){
		return 1;
	}
	var h = data['get_hits']/data['cmd_get'];
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

function memcache_attr(data){
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
	var get_btime = gett(data['uptime']);
	var p = $(div_tpl).append('<div class="uptime" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">运行时间:'+Math.floor(data['uptime']/60)+'分钟'+data['uptime']%60+'秒'+'</div>');
	
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">使用内存:'+Math.floor((data['limit_maxbytes']/1024)/1024)+'MB'+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">总内存:'+Math.floor((data['limit_maxbytes']/1024)/1024)+'MB'+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">读取KB:'+Math.floor((data['bytes_read']/1024))+'KB'+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">写入KB:'+Math.floor((data['bytes_written']/1024))+'KB'+'</div>');
	p = $(p).append('<div class="byte" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">连接次数:'+data['total_connections']+'次'+'</div>');
	p = $(p).append('<div class="byte" title="当这个大于0时,上面的缓存命中率就不正确" style="width:600px;font-size:12px;height:20px;float:left;line-height:20px;border-bottom:1px solid #DFDFDF;">为获取空闲内存而删除的items数:'+data['evictions']+'item'+'</div>');

	
	//end in below
	$('#m_left_attr').append(h_tpl);
	$('#m_left_attr').append(p);
}

function all_var(){
	get_var(function(data){
		//列表显示
		list_var(data);
		cached_query_mzl(data);
		memcache_attr(data);
	});
}
memcache_all();
function memcache_all(){
	all_var();
}

var timer;
function timer_start_and_time(s){
	//console.log(s);
	if('stop'==s){
		clearInterval(timer);
	}else{
		timer = setInterval(function(){
			memcache_all();
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
				<span>服务器地址:<input class="inputtype" type="text" name="local" value="localhost" /></span>
				<span>端口:<input class="inputtype" type="text" name="port" value="11211" /></span>
				<span>间隔刷新时间:<input class="inputtype" type="text" name="time" value="2" />秒</span>
				<span><input type="button" name="run" value="执行" style="height:20px;width:50px;" /></span>
			</form>
		</div>
		
		<div id="main">
			<div id="m_left" style="float:left;width:600px;">
				<div id="m_left_cached" style="overflow:hidden;">
					<h3 style="font-size:25px;hegiht:30px;line-height:25px;text-align:center;border-bottom:1px solid #DFDFDF;">
						MemCache缓存命中率
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

<?php
//加载需要的配置
include(dirname(__FILE__).'/config.php');

//var_dump($_GET);
if(isset($_GET['midoks']) && ('dede' == $_GET['midoks'])){
	weixin_robot_start();
}

//微信机器人服务开始启用
function weixin_robot_start(){
	if(isset($_GET['midoks']) ){//sign
		//微信消息处理类
		include_once(WEIXIN_ROOT_LIB.'weixin_robot.php');
		global $weixin_robot;
		if(!isset($weixin_robot)){
			$weixin_robot = new weixin_robot();
			//验证或返回信息
			$weixin_robot->valid();
			exit;
		}
	}
}
?>

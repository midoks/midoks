<?php
/**
 * 微信机器人程序
 */
//载入配置
include 'config.php';
class plugin_discuz_weixin_robot{

	//架构函数
	public function __construct(){}

	//微信机器人服务开始启用
	public function weixin_robot_start(){
		if(isset($_GET['midoks']) && ('discuz' == $_GET['midoks'])){//sign
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

	public function global_footer(){
		//var_dump($_GET);
		//var_dump('fuck');
		//启动微信机器人
		$this->weixin_robot_start();
		return false;
	}
}

?>

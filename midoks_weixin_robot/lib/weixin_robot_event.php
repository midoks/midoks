<?php
/**
 *	@func 微信事件类
 *	Author: Midoks
 *  Author URI: http://midoks.cachecha.com/
 */
class weixin_robot_event{

	public $cmd;

	//架构函数
	public function __construct($obj){
		$this->cmd = $obj;
	}

	//订阅事件
	public function subscribeEvent(){
		$s = $this->cmd->options['subscribe'];
		if(!empty($s)){
			return $this->cmd->toMsgText($s);
		}
		return $this->cmd->toMsgText('感谢你的对小M博客关注(midoks.cachecha.com),正在开发WP微信机器人插件v2中...');
	}

	//取消订阅时间
	public function unsubscribeEvent(){
		return $this->cmd->toMsgText('谢谢你的使用!!!');
	}

	//上报地址事件|查天气
	public function locationEvent(){
		//基本数据
		$latitude = $this->cmd->info['Location_X'];
		$longitude = $this->cmd->info['Location_Y'];
		$precision = $this->cmd->info['Scale'];
		$label = $this->cmd->info['Label'];

		if(empty($latitude)){
			return $this->cmd->toMsgText('没有获取你的数据...!!!');
		}

		include(WEIXIN_ROOT_API.'weixin_robot_api_weather.php');
		$le = new weixin_robot_api_weather();
		$info = $le->cmd($latitude, $longitude, $precision, $label);
		//$latitude = json_encode($this->cmd->info);
		//return $this->cmd->toMsgText("感谢你提交地址信息!!! +:".$latitude);
		return $this->cmd->toMsgText($info);
	}

	//用户已关注时的事件推送
	public function scanEvent(){
		return $this->cmd->toMsgText('谢谢再次关注!!');
	}



}
?>

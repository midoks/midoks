<?php
/**
 *	@func 接口 用户自动菜单定义
 */
class weixin_robot_event_user{
	
	public $obj = null;

	public $callback = array(
		'0', 'n5', 'h5', 'r5', 'today', '?',
	);

	//预留进行二次开发
	public $self_callback = array(
		
	);//你自定义方法

	public function __construct($obj){
		$this->obj = $obj;
	}

	public function go($key){
		$data = $this->obj->db->weixin_get_menu_data();
		if($data){
			foreach($data as $k=>$v){
				if($key == $v['menu_key']){
					return $this->choose($v['menu_callback']);
				}
			}
		}
		return  $this->obj->helper('key:'.$key."\n".'用户自定菜单响应未定义?');
	}	

	public function choose($case){
		if(in_array($case, $this->callback)){//预定义
			include(WEIXIN_ROOT_LIB.'text/weixin_robot_textreplay.php');
			$text = new weixin_robot_textreplay($this->obj, $case);
			return $text->replay();
		}else if(in_array($case, $this->self_callback)){//预留接口
			return $this->self_choose($case);
		}else{
			return $this->obj->toMsgText($case);
		}
	
	}

	public function self_choose($case){}

}
?>

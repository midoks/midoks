<?php
/**
 *	@func 文本消息处理类
 */
class weixin_robot_textreplay{

	public $obj = null;
	public $kw = '';//关键字

	public $list_cmd = array('0','today','n5', 'h5', 'r5', '?');

	/**
	 *	@func 构造函数
	 *	@param object $obj 模板引用对象
	 *	@param string $kw 关键字
	 */
	public function __construct($obj, $kw){
		$this->obj = $obj;
		$this->kw = $this->convert($kw);
	}

	//响应信息
	public function replay(){
		return $this->wx_result();
	}
	
	//文本结果返回
	public function wx_result(){
		$kw = $this->kw;

		//var_dump($kw);
		if($kw == '?'){
			return $this->obj->toMsgText($this->obj->options['weixin_robot_helper']);//显示帮助信息
		}

		//用户自定义关键字回复
		if($user_cmd = $this->user_keyword_cmd($kw)){
			return $user_cmd;
		}

		//文章信息
		if($wp_cmd = $this->wordpress_cmd($kw)){//执行后,结束
			return $wp_cmd;
		}

		//插件接口调用
		if($wp_plugins = $this->obj->plugins->dealwith('text', $kw)){
			return $wp_plugins;
		}
	
		return $this->obj->helper();//显示帮助信息
	}

	//用户自定义关键字回复
	public function user_keyword_cmd($kw){
		$data = $this->obj->db->weixin_get_relpy_data();
		//var_dump($data);
		if($data){
			$arr = array();
			foreach($data as $k=>$v){
				if('1' == $v['status']){
					if($kw == $v['keyword'] && 'text' == $v['type']){
						if(in_array($v['relpy'], $this->list_cmd)){
							return $this->wordpress_cmd($v['relpy']);
						}else{
							return $this->obj->toMsgText($v['relpy']);
						}
					}else if($kw == $v['keyword'] && 'id' == $v['type']){
						if(in_array($v['relpy'], $this->list_cmd)){//这是为兼容错误
							return $this->wordpress_cmd($v['relpy']);
						}else{
							if(count($idsc = explode(',', $v['relpy']))>1){			
								$data = $this->obj->wp_db->Qids($idsc);
							}else{
								$data = $this->obj->wp_db->Qid(trim($v['relpy'],','));
							}
							return $data;
						}
					}else{
						return false;
					}
				}
			}
			if(empty($arr)){
				return false;
			}
			return $arr;
		}
		return false;
	}

	//wordpress文件信息(主要的)
	public function wordpress_cmd($kw){
		$wp =  $this->obj->wp_db;
		switch($kw){
			case '0'		: $res = $wp->get_category_list();break;
			case 'today'	: $res = $wp->today();break;
			case 'n5'		: $res = $wp->new5();break;
			case 'h5'		: $res = $wp->hot5();break;
			case 'r5'		: $res = $wp->rand5();break;
			case 'p?'		: $res = $wp->total();break;
			default			: $res = $wp->pageView($kw);break;
		}	
		return $res;
	}

	/**
	* 字符串半角和全角间相互转换
	* @param string $str 待转换的字符串
	* @param int $type TODBC:转换为半角；TOSBC，转换为全角
	* @return string 返回转换后的字符串
	*/
	public function convert($str, $type = 'TOSBC'){
		$dbc = array('！', '？', '。');
		$sbc = array('!', '?', '.');
		if($type == 'TODBC'){
			return str_replace($sbc, $dbc, $str); //半角到全角
		}elseif($type == 'TOSBC'){
			return str_replace($dbc, $sbc, $str); //全角到半角
		}else{
			return false;
		}
	}
}
?>

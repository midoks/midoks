<?php
class wpwx_text_exp{

	private $obj = null;

	//构造函数 | init
	public function __construct($obj){
		$this->obj = $obj;
	}

	//开始执行
	//**
	//	如果你是文本处理 $kw 是一个用户关键
	//	如果你是不使用文本 $kw 发送过来的所有信息 (数据) 数组
	///
	public function start($kw){
		if('测试' == $kw){
			return $this->obj->toMsgText('God, 你真的成功了!!!');
		}
		return false;
	}

	
}


?>

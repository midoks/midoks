<?php
/**
 *	@func 数据库类
 */
class DB{

	private $link;//连接资源

	/**
	 * 	@func 架构函数
	 */
	public function __construct(){
		$this->link = mysql_connect('localhost:3306','root','');
		mysql_select_db('shopex',$this->link);
		mysql_query('set names utf8',$this->link);
	}

	/**
	 * @func 数据库查询 | 面向数据库
	 * @param $sql sql语句
	 * @return 错误:false | 正确:返回数据
	 */
	public function query($sql){
		return mysql_query($sql,$this->link);
	}

	/**
	 * @func 数据库查询 | 面向服务器
	 * @param $sql sql语句
	 * @return 错误:false | 正确:返回数据
	 */
	public function squery($sql){
		return mysql_query($sql);
	}


	/**
	 *	@func 析构函数
	 */
	public function __destruct(){
		mysql_close($this->link);
	}
}


?>

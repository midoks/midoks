<?php
/**
 * 所有开发的功能针对MySQL使用
 * author:midoks
 * email: midoks@163.com
 */

class DB{

	private $dbMaster	= NULL;
	private $dbSlave	= NULL;
	private $dbLink		= NULL;
	private $dbConf 	= NULL;

	//单例对象
	private static $_instance  = NULL;

	/**
	 * 构造函数(不可使用)
	 */
	private function __construct(){}


	/**
	 * DB单例模式
	 * @return DB object
	 */
	public static function getInstance(){
		if (!self::$_instance){
			self::$_instance = new self();
		}
		return self::$_instance;
	}

	/**
	 * 注入配置信息
	 * 格式:
	 * ['write' => [
	 *		[
	 *			'host' 		=> '127.0.0.1',
	 *			'port' 		=> '3306',
	 *			'user' 		=> 'root',
	 *			'password'	=> 'root',
	 *			'charset'	=> 'utf8',
	 *			'prefix'	=> 't_',
	 *		],
	 *	],
	 *	'read' => [
	 *		[
	 *			'host' 		=> '127.0.0.1',
	 *			'port' 		=> '3306',
	 *			'user' 		=> 'root',
	 *			'password'	=> 'root',
	 *			'charset'	=> 'utf8',
	 *			'prefix'	=> 't_',
	 *		]
	 *	]]
	 */
	public function injection($conf){
		var_dump($conf);
	}



	public function query($sql){

	}


}

?>
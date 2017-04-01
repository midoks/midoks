<?php
/**
 *	数据库配置文件
 *  (多项目)多主多从配置文件
 */

return array(

	//default表示默认本项目,主要数据库
	'default' => array(
		'write' => array(
			array(
				'host' 		=> '127.0.0.1',
				'port' 		=> '3306',
				'user' 		=> 'root',
				'password'	=> 'root',
				'charset'	=> 'utf8',
				'prefix'	=> 't_',
			),
		),
		'read' => array(
			array(
				'host' 		=> '127.0.0.1',
				'port' 		=> '3306',
				'user' 		=> 'root',
				'password'	=> 'root',
				'charset'	=> 'utf8',
				'prefix'	=> 't_',
			),
		),
	)
);

?>
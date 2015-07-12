<?php
/**
 *	@func 基本管理模型
 */
class PdoModel{

	
	protected 	$linkID = null;
	public 		$config = array();

	/**
	 *	@func 构造函数
	 */
	public function __construct($config){
		//$conf = C('db');
		//parse_str($conf, $config);
		$this->config = $config;
		
		$this->connect();	
	}

	/**
	 *	数据库链接
	 */
	private function connect(){
		$c = $this->config;
		try{  
			$dbh = new PDO('mysql:host='.$c['db_host'].';dbname='.$c['db_name'],
						   	$c['db_user'], 
						  	$c['db_pwd'],
							array(	
								PDO::ATTR_PERSISTENT	=>	false, 	//是否支持长链接
								PDO::ATTR_AUTOCOMMIT	=>	true ) //是否自动提交
							);

			//产生致命错误,PDOException
			$dbh->setAttribute(PDO::ATTR_ERRMODE,PDO::ERRMODE_EXCEPTION);
			$dbh->exec('set names '.$c['db_charset']);
			$this->linkID = $dbh;
		}catch(PDOException $e){  
			echo '数据库连接失败: '.$e->getMessage();exit;
		}
	}

	/**
	 * 事务相关的函数
	 * 默认的MyISAM 不支持事务
	 * InnoDB 支持事务
	 */

	//开始事务
	public function begin(){
		if(!$this->linkID) return false;
		return $this->linkID->beginTransaction();
	}

	//事务回滚
	public function rollback(){
		if(!$this->linkID) return false;
		return $this->linkID->rollBack();
	}

	//事务确认
	public function commit(){
		if(!$this->linkID) return false;
		return $this->linkID->commit();
	}

	/**
	 *	对不支持事务的MyISAM引擎数据库使用锁表
	 */

	/**
	 * 	@func 	锁表
	 *	@param 	$table	表名
	 *	@param	$rw 	[READ|WRITE]
	 *	@ret 	void
	 */
	public function lock($table, $rw = 'WRITE'){
		return $this->linkID->query("LOCK TABLES `{$table}` {$rw}");
	}

	/**
	 * 	@func 	解锁
	 * 	@param	$table 表名
	 * 	@ret 	void
	 */
	public function unlock(){
		return $this->linkID->query('UNLOCK TABLES');
	}

	//其他操作
	
	/**
	 * 	加入表前缀
	 * 	@param 	string $table 表名
	 * 	@param 	string $prefix 表前缀
	 * 	@ret 	stirng
	 */
	public function table($table, $prefix=''){
		if(!empty($prefix)){
			return $prefix.$table;
		}else{
			return $this->config['db_table_prefix'].$table;
		}
	}

	
	/**
	 *  @func 
	 *  
	 */
	public function query($sql, $bind = array(),$mode = PDO::FETCH_ASSOC){
		$trim_sql = trim($sql);
		//查询数据
		if(preg_match( '/^\s*(select|show|describe)/i', $trim_sql)){
			return $this->get_result($trim_sql, $mode);
		}else if(preg_match('/^\s*(insert|delete|update|replace)/i', $trim_sql)){
			//添加数据 更新数据 删除数据 替换数据
			return $this->exec($trim_sql, $bind);
		}
		return false;
	}
	
	//查询操作
	public function get_result($sql,$mode = PDO::FETCH_ASSOC){
		$res = array();
		$rows = $this->linkID->query($sql);
		while($data = $rows->fetch($mode) ){
			$res[] = $data;
		}
		return $res;
	}

	/**
	 *	@func 对数据有变动的操作
	 */
	private function exec($sql, $bind = array()){
		$stmt = $this->linkID->prepare($sql);
		if(!$stmt){
			return $this->linkID->errorInfo();
		}
		$ret = $stmt->execute($bind);
		if(preg_match('/^\s*(insert)/i', $sql)){
			$last_id = $this->insert_last_id();
			return $last_id;
		}else if(preg_match('/^\s*(update|delete|replace)/i', $sql)){
			return $stmt->rowCount();
		}
		return false;
	}

	//最后插入的ID
	public function insert_last_id(){
		 return $this->linkID->lastInsertId();
	}

	//字符串转义
	public function quote($string, $parameter_type = PDO::PARAM_STR){
		return $this->linkID->quote($string, $parameter_type);
	}


	/**
	 *	@func 析构函数
	 */
	public function __destruct(){
		$this->linkID = null;
	}

}
?>

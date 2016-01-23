<?php

/**
 *	热更新代码	
 */
class hotReload{

	public $confg;


	public $dht;

	public function __construct(){		
		$this->config = array(
			'timer'			=>	1, 		//定时器秒
			'self_update' 	=>  3000, 	//文件自更新数据
		);


		include('DHT/DHT.php');
		$this->dht = new DHT();

	}

	//业务逻辑处理(必须存在)[定时器]
	public function worker($serv){
		//爬虫相关
		$this->dht->setServ($serv);
		$this->dht->run();

		$time = date('Y-m-d h:i:s');
		echo "当前时间:{$time}\n";
		//$this->logs($time,'log');
		
		//$this->update($serv);
	}

	//接受的请求信息
	public function onReceive($serv, $fd, $from_id, $data){
		//echo $data."\n";

		$this->dht->setServ($serv);


		$fdinfo = $serv->connection_info($fd, $from_id);
		$this->dht->accept($data, $fdinfo);
		
	}


	//自动更新
	public function update($serv){
		//echo date('Y-m-d h:i:s')."\n";
		//if(date('s') == '00'){
		if(date('s') % 10 == 0){
			echo "update!!!\n";
			$this->logs(date('Y-m-d h:i:s').' -------- update', 'update');
			$serv->reload();
		}
	}

	//记录日志
	public function logs($content, $fileName = 'content'){
		$filename = dirname(__FILE__).'/log/'.$fileName.'-'.date('Y-m-d').'.txt';
		if(!file_exists($filename)){
			@touch($filename);
			@chmod($filename,0777);//权限改为777
		}
		$fp = fopen($filename, "a");
		flock($fp,	LOCK_EX) ;
		fwrite($fp, $content."\n");
		flock($fp, 	LOCK_UN);
		fclose($fp);
	}

}






if(isset($_GET['test']) && $_GET['test'] == '123'){
	$obj = new hotReload();
	$obj->update();
}

?>

<?php


class TimeServer{

	private $serv;
	public $timer;
	private $worker;

	public function __construct(){
		global $argv;
		if(!empty($argv)){
			if('start' == $argv[1]){
				$this->start();
			}else if('stop' == $argv[1]){
				$this->stop();
			}else if('reload' == $argv[1]){
				$this->reload();
			}
		}
		//var_dump($argv);
		//var_dump(func_get_args());
	}

	public function start_test(){
		$process = new swoole_process(array($this, 'start_deamon'), true);
		$pid = $process->start();
		$this->logs($pid, 'deamon');
		swoole_process::wait();
	}

	//start deamon	
	public function start(){
		echo "start\n";
		$this->serv = new swoole_server('0.0.0.0', 9501);
		$this->serv->addlistener("0.0.0.0", 9504, SWOOLE_SOCK_UDP);
		$this->serv->set(array(
			'worker_num'	=> 1,
			'daemonize'		=> false, //正式环境应该是true
			'max_conn'		=> 256,
			'max_requst'	=> 50,
			'dispatch_mode'	=> 1,
			//'debug_mode'	=> 1,
			'heartbeat_check_interval'	=> 30,
			//'log_file'		=> dirname(__FILE__).'/swoole_err.log'
		));


		$this->serv->on('WorkerStart', function($serv, $worker_id){
			echo "OnWorkerStart\n";

			include('hot_reload.php');
			$this->worker = null;
			$this->worker = new hotReload();
			
			if( 0 == $worker_id){
				$serv->addtimer(1000);
			}

		});

		$this->serv->on('WorkerStop', function(){
			echo("OnWorkerStop\n");
			if(function_exists('opcache_reset')){
				opcache_reset();
			}
			
			if(function_exists('eaccelerator_clear')){
				eaccelerator_clear();
				eaccelerator_clean();
				eaccelerator_purge();
			}
		});

		$this->serv->on('Connect', array($this, 'onConnect'));
		$this->serv->on('Receive', array($this, 'onReceive'));
		$this->serv->on('Close', array($this, 'onClose'));
		$this->serv->on('Timer', array($this, 'onTimer'));

		
		$this->serv->start();
	
	}

	//stop
	public function stop(){
		echo "stop\n";
		$client = new swoole_client(SWOOLE_SOCK_TCP, SWOOLE_SOCK_ASYNC);
		$client->on('connect', function($cli){
			echo "connect\n";
			$cli->send('stop');
		});

		$client->on('receive', function($cli, $data){
			echo "receive\n";
		});

		$client->on('error', function($cli){
			echo "error\n";
		});

		$client->on('close', function($cli){
			echo "close\n";
		});

		$client->connect('0.0.0.0', 9501, 0.5);

	}

	public function reload(){
		echo "reload\n";
		$client = new swoole_client(SWOOLE_SOCK_TCP, SWOOLE_SOCK_ASYNC);
		$client->on('connect', function($cli){
			echo "connect\n";
			$cli->send('reload');
		});

		$client->on('receive', function($cli, $data){
			echo "receive\n";
		});

		$client->on('error', function($cli){
			echo "error\n";
		});

		$client->on('close', function($cli){
			echo "close\n";
		});

		$client->connect('0.0.0.0', 9501, 0.5);
	}

	
	public function onConnect($serv, $fd){
		$this->logs("client {$fd} connect", 'content');
	}
	
	public function onReceive($serv, $fd, $from_id, $data){
		if('stop' == $data){
			$serv->clearTimer($this->timer);
			$serv->close($fd);
			$serv->shutdown();
		}else if('reload' == $data){
			//重新加载
			$serv->reload();
		}else{
		
			$this->worker->onReceive($serv, $fd, $from_id, $data);
		}
	
		//echo "Get Message From Client {$fd}:{$data}";
	}

	public function onClose($serv, $fd, $from_id){
		$this->logs("Client {$fd} close Connection", 'close');
	}

	public function onTimer($serv, $interval){
		switch($interval){
			case 1000: $this->worker($serv);break;
		}
	}

	public function worker($serv){
		$this->worker->worker($serv);
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
		fwrite($fp, $content."\t\n");
		flock($fp, 	LOCK_UN);
		fclose($fp);
	}
}

new TimeServer();
?>

<?php
/**
 * DHT协议相关
 */
//http://www.bittorrent.org/beps/bep_0005.html

class DHT{

	public $hash;
	public $dht;
	public $bencode;
	public $serv;

	public $list_node = array();
	public $list_node_limit;


	public $self_id;

	public function __construct(){
		$this->list_node_limit = 100;

		$this->self_id = $this->random_id();

		$this->hash = array(
			array('host' => 'router.bittorrent.com', 	'port'=>'6881'),
			array('host' => 'dht.transmissionbt.com', 	'port'=>'6881'),
			array('host' => 'router.utorrent.com', 		'port'=>'6881'),	
		);

		include('bencode.php');
		$this->bencode = new bencode();
		
		$e = $this->getList();
		if($e){
			$this->list_node = $this->getList();
		}
	}

	
	public function setServ($serv){
		$this->serv = $serv;
	}

	/**
	 * 向对端发送数据
 	 * @param  array $msg     要发送的数据
 	 * @param  array $address 对端链接信息
 	 * @return void
 	 */
	public function send_krpc($msg, $address){
		if(empty($msg)){
			echo "发送消息不能为空\n";
			return false;
		}

		if(!isset($this->serv)){
			echo "对象为空\n";
			return false;
		}

	 	$this->serv->sendto($address[0], $address[1], $this->bencode->en($msg));
	}

	/**
     * 对nodes列表编码
     * @param  mixed $nodes 要编码的列表
     * @return string        编码后的数据
     */
	public function en_nodes($nodes){
		// 判断当前nodes列表是否为空
        if(count($nodes) == 0){
			return $nodes;
		}
        $n = '';
        // 循环对node进行编码
        foreach($nodes as $node){
			$n .= pack('a20Nn', $node['nid'], ip2long($node['ip']), $node['port']);
		}
        return $n;
	}

	/**
     * 对nodes列表解码
     * @param  string $msg 要解码的数据
     * @return mixed      解码后的数据
     */
    public function de_nodes($msg){
        // 先判断数据长度是否正确
        if((strlen($msg) % 26) != 0){
			return array();
		}
		
        $n = array();
        // 每次截取26字节进行解码
        foreach(str_split($msg, 26) as $s){
            // 将截取到的字节进行字节序解码
			$r = unpack('a20nid/Nip/np', $s);
			
			$tmp = [
				'nid' 	=> $r['nid'],
				'ip'	=> long2ip($r['ip']), 
				'port'	=> $r['p']
			];
			$n[] = $tmp;
        }
        return $n;
	}


	//如果没有就伪造
	public function _get_nodes(){
		$list = array(
			array('host' => 'router.bittorrent.com', 	'port'=>'6881'),
			array('host' => 'dht.transmissionbt.com', 	'port'=>'6881'),
			array('host' => 'router.utorrent.com', 		'port'=>'6881'),	
		);

		foreach($list as $k=>$v){
			$list[$k]['nid'] = $this->entropy(20);
		}

		return $list;
	}

	public function get_nodes($len = 8){
    	$table  = $this->list_node;
    	if(count($table) <= $len){
			return $this->_get_nodes();
			//return $table;
		}
    	$nodes = array();
		
		for($i=0; $i<$len; $i++){
    	    $nodes[] = $table[mt_rand(0, count($table) - 1)];
    	}
    	return $nodes;
	}

	//查找附近的节点
	public function get_neighbor($target, $nid){
		return substr($target, 0, 10).substr($nid, 10, 10);
	}

	//产生ID
	public function entropy($len = 4){
		$s = '';
		for($i=0;$i<$len;$i++){
			$r = mt_rand(0, 255);
			$s .= chr($r);
		}
		return $s;
	}

	public function random_id(){
		return $this->entropy(20);
	}

	//查找节点
	public function find_node($address, $id = null){
		//var_dump($address);
		$nid = $this->self_id;
		//var_dump($nid);
		if(is_null($id)){
			$mid = $nid;
		}else{
			$mid = $this->get_neighbor($id, $nid);
		}

		//var_dump($nid, $mid);
		$msg = [
			't' => $this->entropy(2),
			'y'	=> 'q',
			'q' => 'find_node',
			'a' => [
				'id' 		=> $nid,
				'target' 	=> $mid,
			],
		];

		$this->send_krpc($msg, $address);
	}

	//加入DHT
	public function joinDHT(){
		$node = $this->hash;
		foreach($node as $k=>$v){
			$this->find_node(array(gethostbyname($v['host']), $v['port']));
		}
	}

	//循环数据
	public function auto_find_node(){
		//while(count($this->list_node)){
			foreach($this->list_node as $k=>$v){
				$nid = $v['nid'];
				if(empty($nid)){
					$nid = $this->random_id();
				}

				//echo($nid."\n");

				$this->find_node(array($v['ip'], $v['port']), $nid);
				usleep(100);
			}
		//}
	}

	//开始运行
	public function run(){
		$node_count = count($this->list_node);
		echo "当前数据桶:{$node_count}\n";
		echo "当前服务器共有:".count($this->serv->connections). " 个连接\n";
		
		if($node_count == 0){
			$this->joinDHT();
			//echo 'join DHT run'."\n";
		}else{
			//echo 'auto_find_node run'."\n";
			$this->joinDHT();
			$this->auto_find_node();
		}

		$this->putList($this->list_node);
	}

	/**
	 *   接受数据操作
	 */
	public function accept($data, $fdinfo){
		//echo '接受到数据!!!'."\n";
		$info = $this->bencode->de($data);
		//var_dump($info);
		//var_dump($fdinfo);
		//echo($info['y']."\n");
		//
		if(strlen($info['y']) == 1){

			if('r' == $info['y']){
				if(array_key_exists('nodes', $info['r'])){
					$this->accept_response($info, array($fdinfo['remote_ip'], $fdinfo['remote_port']));
				}
			}else if('q' == $info['y']){
				$this->accept_request($info, array($fdinfo['remote_ip'], $fdinfo['remote_port']));
			}else if('e' == $info['y']){
				//错误信息
				$err = $info['e'];
				$this->logs(json_encode($err) . json_encode(array($fdinfo['remote_ip'], $fdinfo['remote_port'])), 'error');
			}

		}else{
			var_dump($info['y']);
		}


		return false;
	}


	//接受请求信息
	public function accept_request($msg, $address){
		//echo("request\n");
		switch($msg['q']){
			case 'ping': 			$this->on_ping($msg, $address);break;
			case 'find_node': 		$this->on_find_node($msg, $address);break;
			case 'get_peers':		$this->on_get_peers($msg, $address);break;
			case 'announce_peer':	$this->on_announce_peer($msg, $address);break;
			default:return false;
		}
	}

	//接受并相应信息
	public function accept_response($msg, $address){
		//echo("response\n");

		//检查是否正确
		if(!isset($msg['r']['nodes']) || !isset($msg['r']['nodes'][1])){
			return false;
		}
		
		// 对nodes数据进行解码
		//$this->logs($msg['r']['nodes'],'source_nodes');
		$nodes = $this->de_nodes($msg['r']['nodes']);
		
		if(!empty($nodes)){
			// 对nodes循环处理
			foreach($nodes as $node){// 将node加入到路由表中
				$this->append($node);

				//var_dump($node['nid']);

				$node['nid'] = '';
				$this->logs(json_encode($node), 'nodes');
			}
		}
	}

	//处理ping请求
	public function on_ping($msg, $address){
		echo("on_ping\n");
		$sid = $this->self_id;

		$id = $msg['a']['id'];

		$msg = [
			't' => $msg['t'],
			'y' => 'r',
			'r' => [
				'id' => $sid,
			],
		];

		
		$node = [
			'nid' 	=> $id,
			'ip'  	=> $address[0],
			'port' 	=> $address[1],
		];

		$this->append($node);

		$this->send_krpc($msg, $address);
	}


	//处理find_node
	public function on_find_node($msg, $address){
		echo("on_find_node\n");
		$sid = $this->self_id;
		
		$nodes = $this->get_nodes(16);

		$id = $msg['a']['id'];

		$msg = [
			't' => $msg['t'],
			'y' => 'r',
			'r' => [
				'id' => $sid,
				'nodes' => $this->en_nodes($nodes),
			]
		];

		$node = [
			'nid' 	=> $id,
			'ip'  	=> $address[0],
			'port' 	=> $address[1],
		];
		$this->append($node);

		$this->send_krpc($msg, $address);
	}


	public function on_get_peers($msg, $address){
		echo("on_get_peers\n");
		$sid = $this->self_id;

		$infohash = $msg['a']['info_hash'];
		
		$id = $msg['a']['id'];

		$node = [
			'nid' 	=> $id,
			'ip'  	=> $address[0],
			'port' 	=> $address[1],
		];
		$this->append($node);

		$msg = [
			't' => $msg['t'],
			'y' => 'r',
			'r' => [
				'id' 	=> $sid,
				'nodes' => $this->en_nodes($this->get_nodes()),
				'token' => substr($infohash, 0, 2),
			]
		];
	
		$this->send_krpc($msg, $address);
	}

	public function on_announce_peer($msg, $address){
		echo("on_announce_peer\n");
		$sid = $this->self_id;

		// 获取infohash
   		$infohash	= $msg['a']['info_hash'];
    	// 获取token
    	$token		= $msg['a']['token'];
    	// 获取node id
    	$id 		= $msg['a']['id'];

		// 生成回复数据
    	$msg = [
        	't' => $msg['t'],
        	'y' => 'r',
       		'r' => [
       	    	 'id' => $sid,
    	    ]
		];

		echo(strtoupper(bin2hex($infohash))."\n");

		//if(substr($infohash, 0, 2) == $token){
		//$this->logs(date('Y-m-d H:i:s') .' 获取到info_hash'. strtoupper(bin2hex($infohash)), 'hash');
		//}

		$this->send_krpc($msg, $address);
	}


	/**
	 * 添加node到路由表
	 * @param  array 	$node 	node模型
	 * @return boolean      	是否添加成功
	 */
	public function append($node){
		//echo "append\n";
		
		$sid = $this->self_id;
		// 检查是否为自身node id
		if($node['nid'] == $sid){
			return false;
		}

		// 检查node是否已存在
		if($this->append_check_exists($node)){
			//echo "exists\n";
			return false;
		}

		$limit = $this->list_node_limit;
		$list_count = count($this->list_node);


		if($list_count > $limit){
			$offset = $list_count - $limit;
			$this->list_node = array_slice($this->list_node, $offset, $limit);

		}

		// 如果路由表中的项达到200时, 删除第一项
		if(count($this->list_node) >= $limit){
			array_shift($this->list_node);
		}
		//echo "append ok\n";
		
		$this->list_node[] = $node;
	}

	private function append_check_exists($node){
		if(!empty($this->list_node)){
			foreach($this->list_node as $k=>$v){
				if($v['ip'] == $node['ip'] && $v['port'] == $node['port']){
					return true;
				}
			}
		}
		return false;
	
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

	public function putList($content){
		$filename = dirname(__FILE__).'/log/list.txt';
		
		foreach($content as $k=>$v){
			$content[$k]['nid'] = base64_encode($v['nid']);
		}
		$json = json_encode($content);
		file_put_contents($filename, $json);
	}

	public function getList(){
		$filename = dirname(__FILE__).'/log/list.txt';
		$list = file_get_contents($filename);

		if(empty($list)){
			return false;
		}

		$list = json_decode($list, true);

		foreach($list as $k=>$v){
			$list[$k]['nid'] = base64_decode($v['nid']);
		}

		return $list;
	}

}


if(isset($_GET['test']) && $_GET['test'] == '123'){
	$obj = new DHT();
	echo $obj->entropy();
}
?>

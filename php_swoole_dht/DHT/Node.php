<?php
/**
 * DHT协议相关
 */
//http://www.bittorrent.org/beps/bep_0005.html

class Node{
	
	public $ip;
	public $port;

	public $nid;

	public function __construct($nid, $ip, $port){
		$this->nid 	= $nid;
		$this->ip 	= $ip;
		$this->port = $port;
	}

	public function to_array(){
        return array('nid' => $this->nid, 'ip' => $this->ip, 'port' => $this->port);
    }
}
?>

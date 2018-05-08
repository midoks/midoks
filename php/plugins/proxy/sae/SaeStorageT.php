<?php
class SaeStorageT{

	public $linkID = null;
	public $domain;

	public $config = array();

	public function __construct($name){
		$this->linkID = new SaeStorage();
		$this->domain = $name;

		$time = 60*60*12;
		$this->config = array(
			'timeout' => $time,//秒
			'filesize'=> 3072,
		);
	
	}

	public function get($fn){

		if(!$this->linkID->fileExists($this->domain, $this->uri($fn))){
			return false;
		}else{
			//判断是否过期
			$a = $this->linkID->getAttr($this->domain, $this->uri($fn));
			$ftime = $a['datetime'];
			$diff_time = time() - $ftime;
			if($diff_time < $this->config['timeout']){
				$this->HeaderCache($fn);
				$data = $this->linkID->read($this->domain, $this->uri($fn));
				return $data;
			}else{
				return false;
			}
		}
		return false;	
	}

	public function HeaderCache($fn){
		$type = array(
			'css' => 'text/css',
			'js'  => 'application/x-javascript', 
			'png' => 'image/png', 
			'ico' => 'image/x-icon',
			'jpg' => 'image/jpeg',
			'gif' => 'image/gif',
			'swf' => 'application/x-shockwave-flash',
		   	'htm' => 'text/html', 
			'html' => 'text/html',		
		);

		foreach($type as $k=>$v){
			if(preg_match("/.{$k}/", $fn)){
				
				header("Content-type: ".$v);
				header("Connection: close"); 
				header("Cache-Control: max-age=30000"); 
				header("Cache-Control: public");
  				header("Pragma: cache");

  				$offset = 30*60*60*24; // cache 1 month
  				$ExpStr = "Expires: ".gmdate("D, d M Y H:i:s", time() + $offset)." GMT";
  				header($ExpStr);
				//echo 'ok!!';
			}
		}
		
	
	}

	public function uri($fn){
		$fn = str_replace(array(':','/','.','%','?','!','@','#',
			'$','^','*','~','"','\'','{','}','\\','=',
			',','.','?','/',';','<','>','&','(',')'),'_',$fn);
		return $fn;
	}

	public function save($fn, $value){
		$fn = $this->uri($fn);
		if(strlen($value) > $this->config['filesize']){
			$this->linkID->write($this->domain, $fn, $value);
		}
	}
}
?>

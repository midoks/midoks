<?php
class MdAliOSS{

	public $linkID = null;

	public function __construct($name){
		$this->linkID = Alibaba::Stroage($name);

		
		$time = 60*60*6;
		$this->config = array(
			'timeout' => $time,//秒
			'filesize'=> 3072,
		);
		
	}

	public function get($key){
		$fn = $this->uri($key);
		if($this->linkID->fileExists($fn)){
			return false;
		}else{
			$data = $this->linkID->get($fn);
			return $data;
		}
		return false;
	}

	public function uri($fn){
		$fn = str_replace(array(':','/','.','%','?','!','@','#',
			'$','^','*','~','"','\'','{','}','\\','=',
			',','.','?','/',';','<','>','&','(',')'),'_',$fn);
		return $fn;
	}
	

	public function save($key, $value){
		$fn = $this->uri($key);
		return $this->saveText($fn, $value, $this->config['timeout']);
	}
}
?>

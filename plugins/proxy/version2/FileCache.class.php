<?php
/*--------------------------------------------------
 *	1.文件缓存类
 ---------------------------------------------------------*/
define('CACHE_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
define('CACEH_FN', CACHE_ROOT.'cache/');
class FileCache{

	public $config = array(
		'time'	=>	7200,			//保存的时(通用保存时间)
		'size'	=>	'20',			//文件大小[以k为单位]
		'suffix'=> 	'.txt',			//文件的后缀名
		'path'	=> 	CACEH_FN,		//文件缓存路径
		'cachetime' => array(//单位s/秒
			'js' 	=> 	360000,
			'css'	=>	360000,
			'jpg'	=>	360000,
			'swf'	=>	360000,
			'ico'	=>	360000,
			'png'  	=>  360000,
			'gif'	=>	360000,
			'swf' 	=>  360000,
			'html'	=>	7200,
		),
		//'local' => LOCAL_FILE,
	);//基本的配置

	//特殊URI处理
	public function uri($fn){
		$fn = str_replace(array(':','.','%','?','!','@','#',
			'$','^','*','~','"','\'','{','}','\\','=',
			',','.','?','/',';','<','>','&','(',')'),'_',$fn);
		return $fn;
	}

	//常见header文件匹配
	public function mimeTypeMatch($url){
		$allow = array(
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
		foreach($allow as $k=>$v){
			if(preg_match("/.{$k}/", $url)){
				//var_dump($v);
				//if(in_array($k, array('js', 'css'))){
				//	header('Cache-Control: max-age='.(24*60*60)); 
				//}
				return $allow[$k];
			}
		}
		return false;
	}
	
	/*---------------------------------------
	 * @func 获取值
	 * @param $name 	$ 相当于数据中key值 $
	 ---------------------------------------*/
	public function get($name){
		//header
		if($_SERVER['REQUEST_URI']){
			$header = $this->mimeTypeMatch($_SERVER['REQUEST_URI']);
			//var_dump($h);exit;
			if($header){
				header("Content-type: ".$header);
			}
		}

		$uri = $this->uri($name);
		$config = $this->config;

		//特殊类型保存
		$ft = $this->fsave_type();
		if($ft){
			$dir = $ft['dir'];
			if(!file_exists($dir)){
				$this->mkdir_p($dir);
			}

			$fn = $dir.'/'.$ft['name'];
			if(!file_exists($fn)){
				return false;
			}

			$type = $config['cachetime'];
			$type_name = array_keys($type);

			if(in_array($ft['type'], $type_name)){
				$ftime = filemtime($fn);//文件时间
				$ntime = time();//当前时间
				if(($ftime + $type[$ft['type']]) > $ntime){
					$value = file_get_contents($fn);
				}

				if(!empty($value)){
					return $value;
				}
			}
		}

		//通用
		$fn = $config['path'].$uri.$config['suffix'];
		
		if(file_exists($fn)){
			$ftime = filemtime($fn);//文件时间
			$ntime = time();//当前时间
			if((($ftime+$this->config['time']) > $ntime)){
				//$data = file_get_contents($fn);
				$data = $this->fgc($fn);
				if(strlen($data)<1024){
					@unlink($fn);
				}
				return $data;
				
			}
			return false;
		}else{
			return false;
		}
	}
	

	/*------------------------------------------
	 *	@func 存储值
	 * 	@param $name 	$ 文件名 $
	 * 	@param $value 	$ 文件内容 $
	 ------------------------------------------*/
	public function fsave($name, $value){
		$uri = $this->uri($name);
		$config = $this->config;

		if(strlen($value) < 1024)
			return false;
		

		//特殊类型保存
		$ft = $this->fsave_type();
		if($ft){
			$dir = $ft['dir'];
			if(!file_exists($dir) ){
				$this->mkdir_p($dir);
			}
			$type = $config['cachetime'];
			$type = array_keys($type);
			if(in_array($ft['type'], $type)){
				file_put_contents($dir.'/'.$ft['name'], $value);
				return true;
			}	 
		}
		
		$fn = $config['path'].$uri.$config['suffix'];
		file_put_contents($fn, $value);
		return true;
	}

	//获取文件名 and 文件类型
	public function gfile(){
		$uri = $_SERVER['REQUEST_URI'];


		$dir = dirname($uri);
		if('\\' == $dir){
			$dir = CACHE_ROOT;
		}else{
			$dir = CACHE_ROOT.trim($dir, '/');
		}
		$uri = parse_url($uri);
		$uri = $uri['path'];
		$uri = basename($uri);
		
		return array(
			'name'=>$uri,
			'dir' => $dir,
			'type'=>$this->gfiletype($uri),
		);
	}

	//获取文件后缀名
	public function gfiletype($fn){
		$f = explode('.', $fn);
		$c = count($f);
		if($c>1){
			$c = $c - 1;
		}else{
			return false;
		}
		return $f[$c];
	}

	//保存特殊文件
	private function fsave_type(){
		$gft = $this->gfile();
		foreach($gft as $k=>$v){
			if(!$v)
				return false;
		}
		return $gft;
	}

	public function fgc1($fn){
		$handler = fopen($fn ,'r');
		$size = filesize($fn);
		$str = fread($fn, $handler);
		fclose($handler);
		return $str;
	}

	public function fgc($fn){
		ob_start();
		readfile($fn);
		$str = ob_get_clean();
		return $str;
	}

	//安全创建文件夹
	public function mkdir_p($path, $mode=0777){
		$path = '/'.trim($path, '/');
		$list = explode('/', $path);
		array_pop($list);
		$dir = '/'.implode('/', $list);
		//echo $dir, "\r\n";
		if(file_exists($dir)){
			mkdir($path, $mode);
		}else{
			$this->mkdir_p($dir, $mode);
			mkdir($path, $mode);
		}
	}
	
}
?>

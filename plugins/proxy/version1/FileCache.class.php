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
			'html'	=>	7200,
		),
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
			$h = $this->mimeTypeMatch($_SERVER['REQUEST_URI']);
			//var_dump($h);exit;
			if($h){
				header("Content-type: ".$h);
			}
		}

		//特殊类型保存
		$ft = $this->fsave_type();
		if($ft){
			$dir = CACEH_FN.$ft['type'];
			if(!file_exists($dir)){
				mkdir($dir);
			}

			$fn = $dir.'/'.$ft['name'];
			if(!file_exists($fn)){
				return false;
			}

			$type = $this->config['cachetime'];
			$type_name = array_keys($type);
			//var_dump($type);
			if(in_array($ft['type'], $type_name)){
				
				$ftime = filemtime($fn);//文件时间
				$ntime = time();//当前时间
				//echo($ftime + $type[$ft['type']]),"\n";
				//echo $ntime;
				if(($ftime + $type[$ft['type']]) > $ntime){
					//echo 'dd';
					$value = file_get_contents($fn);
				}

				if(!empty($value)){
					return $value;
				}
			}
		}

		//通用
		$uri = $this->uri($name);
		$fn = $this->config['path'].$uri.$this->config['suffix'];
		
		if(file_exists($fn)){
			$ftime = filemtime($fn);//文件时间
			$ntime = time();//当前时间
			if((($ftime+$this->config['time']) > $ntime)){
				return file_get_contents($fn);
			}
			return false;
		}else{
			return false;
		}
	}


	//获取文件名 and 文件类型
	public function gfile(){
		$uri = $_SERVER['REQUEST_URI'];
		$uri = parse_url($uri);
		$uri = $uri['path'];
		$uri = basename($uri);
		
		return array(
			'name'=>$uri,
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
			$c = 1;
		}
		return $f[$c];
	}

	/*------------------------------------------
	 *	@func 存储值
	 * 	@param $name 	$ 文件名 $
	 * 	@param $value 	$ 文件内容 $
	 ------------------------------------------*/
	public function fsave($name, $value){
		//特殊类型保存
		$ft = $this->fsave_type();
		if($ft){
			$dir = CACEH_FN.$ft['type'];
			if(!file_exists($dir) ){
				mkdir($dir);
			}

			$type = $this->config['cachetime'];
			$type = array_keys($type);
			//var_dump($type);
			if(in_array($ft['type'], $type)){
				file_put_contents($dir.'/'.$ft['name'], $value);
				return true;
			}

			 
		}

		$uri = $this->uri($name);
		$fn = $this->config['path'].$uri.$this->config['suffix'];
		file_put_contents($fn, $value);
		return true;
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
	
}
?>

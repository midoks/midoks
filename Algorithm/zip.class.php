<?php
/**
 * 	@func	zip加压处理类
 *	@author midoks
 *	@blog midoks.cachecha.com
 *	@mail midoks@163.com
 */
class zip{

	public $datasec = array();
	public $ctrl_dir = array();
	public $eof_ctrl_dir = "\x50\x4b\x05\x06\x00\x00\x00\x00";
	public $old_offset = 0;

	/**
	 *	@func 压缩文件[目录]
	 *	@param string $dirs 目录
	 *	@param string $zipname 压缩后的文件
	 *	@param boolean 是否删除源文件
	 */
	public function compress($dir, $zipname, $drop = false){
		if(substr($dir, -1) != '/'){//目录的路径
			$dir = ($dir == '') ? './' : $dir.'/';
		}

		if(@function_exists('gzcompress')){
			$curdir = getcwd();//当前的目录
			if(is_array($dir)){//是否是数组
				$filelist = $dir;
			}else{//获取整个目录文件
				$filelist = $this->get_filelist($dir);
			}
			if((!empty($dir)) && (!is_array($dir)) && (file_exists($dir))){
				//不为空 && 不是数组 && 文件存在
				chdir($dir);
			}else{
				chdir($curdir);
			}	
			if(count($filelist)>0){
				foreach($filelist as $filename){
					$fp = fopen($filename, 'rb');
					$size = filesize($filename);
					if($size>0){
						$content = fread($fp, $size);
					}else{
						continue;//跳出本次循环
					}
					fclose($fp);

					if(is_array($dir)){
						$filename = basename($filename);
					}
					//var_dump($content);
					$this->add_files($content, $filename);

					if($drop){
						@unlink($filename);
					}
				}
			}

			$out = $this->files();
          	//return $out;//此处为应用做出的修改
          	chdir($curdir);
			$fp = fopen($zipname, 'wb');
			fwrite($fp, $out, strlen($out));
			fclose($fp);
			return 1;
		}else{
			return 0;
		}
	}

	/**
	 * @func 	获取文件列表
	 * @param 	string $dir 目录文件
	 * @return 	array  返回文件列表
	 */
	public function get_filelist($dir){
		$file = array();
		$dir = rtrim($dir, '/');
		if(file_exists($dir)){
			$args = func_get_args();
			$pref = isset($args[1]) ? $args[1] : '';
			$od = opendir($dir); 
			while(($files = readdir($od)) !== false){
				if($files == '.' || $files == '..'){
				}else{
					if(is_dir($dir.'/'.$files))
						$file = array_merge($file, $this->get_filelist($dir.'/'.$files, $pref.$files.'/'));
					else
						$file[] = $pref.$files;
				}
			}
			closedir($od);
		}
		return $file;
	}

	/**
	 *	@func 把unix时间戳转化为一个4个字节DOS日期和时间格式。
	 *	(2个高字节,允许2个低字节比较)
	 *	@param int 当前的unix时间戳
	 *	@return int 返回当前的日期(4个字节的DOS格式)
	 */
	public function unix2DosTime($unixTime = 0){

		$timearray = ($unixTime == 0) ? getdate() : getdate($unixTime);
        if ($timearray['year'] < 1980) {
            $timearray['year']    = 1980;
            $timearray['mon']     = 1;
            $timearray['mday']    = 1;
            $timearray['hours']   = 0;
            $timearray['minutes'] = 0;
            $timearray['seconds'] = 0;
        }
        return (($timearray['year'] - 1980) << 25) | ($timearray['mon'] << 21) | ($timearray['mday'] << 16) |
                ($timearray['hours'] << 11) | ($timearray['minutes'] << 5) | ($timearray['seconds'] >> 1);
	}

	/**
	 *	@func 把文件添加到压缩文档中
	 *	@param string 文件内容
	 *	@param string 压缩文档中的文件名字(也有可能是文件的路径)
	 *	@param int 当前的时间戳
	 */
	public function add_files($data, $name, $time = 0){
		$name = str_replace('\\', '/', $name);
		$dtime = dechex($this->unix2DosTime($time));
		$hexdtime = '\x' . $dtime[6] . $dtime[7]
                  . '\x' . $dtime[4] . $dtime[5]
                  . '\x' . $dtime[2] . $dtime[3]
				  . '\x' . $dtime[0] . $dtime[1];

		eval('$hexdtime = "'. $hexdtime . '";');

		$fr = "\x50\x4b\x03\x04";
        $fr .= "\x14\x00";						//版本需要提取
        $fr .= "\x00\x00";      				//根目标位标志
		$fr .= "\x08\x00";      				//压缩方法
		$fr .= $hexdtime;						//最后修改的时间和日期	

		// "本地文件头"部分
		$unc_len = strlen($data);
		$crc     = crc32($data);
		$zdata   = gzcompress($data);
		$c_len   = strlen($zdata);
		$zdata   = substr(substr($zdata, 0, strlen($zdata)-4), 2);
		$fr     .= pack('V', $crc);				//crc32
		$fr		.= pack('V', $c_len);			//压缩文件大小
		$fr		.= pack('V', $unc_len);			//解压缩文件大小
		$fr		.= pack('v', strlen($name));	//文件大小
		$fr		.= pack('v', 0);				//额外字段长度
		$fr		.= $name;						//文件名字

		//"文件数据"部分
		$fr		.= $zdata;

		//"数据描述符"部分(可选但必要的档案是不作为文件)
		$fr 	.= pack('V', $crc);				//crc32
		$fr		.= pack('V', $c_len);			//压缩文件大小数据
		$fr		.= pack('V', $unc_len);			//解压缩文件大小数据

		//加入这项队列
		$this->datasec[] = $fr;
		$new_offset 	 = strlen( implode('', $this->datasec) );

		//现在添加到中央目录记录
		$cdrec = "\x50\x4b\x01\x02";
        $cdrec .= "\x00\x00";                	//版本信息
        $cdrec .= "\x14\x00";                	//版本提取
        $cdrec .= "\x00\x00";                	//根目标位标志
		$cdrec .= "\x08\x00";                	//压缩方法
		$cdrec .= $hexdtime;				 	//最后修改时间和日期
	
		$cdrec .= pack('V', $crc);           	//crc32
        $cdrec .= pack('V', $c_len);         	//压缩文件大小数据
        $cdrec .= pack('V', $unc_len);       	//解压缩文件大小数据
        $cdrec .= pack('v', strlen($name)); 	//文件长度
        $cdrec .= pack('v', 0);             	//额外长度字段长度
        $cdrec .= pack('v', 0);             	//文件注释长度
        $cdrec .= pack('v', 0);             	//磁盘数字开始
        $cdrec .= pack('v', 0);             	//内部文件属性
        $cdrec .= pack('V', 32);            	//外部文件属性 - "存档"位集
	
		$cdrec .= pack('V', $this->old_offset);	//本地头的相对偏移
		$this->old_offset = $new_offset;
		$cdrec .= $name;

		//可选的额外字段，文件注释这里保存到中央目录
		$this->ctrl_dir[] = $cdrec;
	}

	/**
	 *	@func 返回打印文件
	 *	@return string 返回压缩文件
	 */
	public function files(){
		$data = implode('', $this->datasec);
		$ctrldir = implode('', $this->ctrl_dir);
		return $data.
			$ctrldir.
			$this->eof_ctrl_dir.				//总数为＃项目,该磁盘上的
			pack('v', sizeof($this->ctrl_dir)).	//总的项目整体
			pack('v', sizeof($this->ctrl_dir)).	//中央目录的大小
			pack('V', strlen($ctrldir)).		//偏移量开始的中央目录
			pack('V', strlen($data)).			//zip文件注释长度。
			"\x00\x00";
	}
}
?>

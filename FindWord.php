<?php
/*---------------------
 * @func $ 查找word方法 $
 * 2012-11-27日升级
 * 遍历目录,查找文件中的单词位置
 * 2013-11-21 修复一个bug
 ---------------------*/
function FindWord($file,$word){
	$fp = fopen($file,'r');
	$count = 1;
	while(!feof($fp)){
		$data = fgets($fp,1024);
		$r = strpos($data,$word);
		if($r){
			echo $word,'在第'.$count.'行:','第'.$r.'个单词:','发现了','<br>';
		}
		++$count;
	}
	fclose($fp);
}
/**
 *	遍历文件
 *	@param string $name 文件名
 *	@return array
 */	
function dir_name($name){
	$name = rtrim($name,'/').'/';
	$fp = opendir($name);
	$arr = array();
	while($n = readdir($fp))
		if($n=='.'|| $n=='..'){
		}else if(is_dir($name.$n)){
			$arr[]= dir_name($name.$n);
		}else if(is_file($name.$n)){
			$arr[] = $name.$n;
		}
	closedir($fp);
	return $arr;
}

/**
 *	2变1维数组
 *	@param array $arr 2维数组
 *	@return array 1维数组
 */
function _3to2($arr){
	foreach($arr as $k=>$v){
		if(!is_array($v)){
			continue;
		}
		//array_splice($arr,$k);
		foreach($v as $v1){	
			$arr[] = $v1;
		}
	}
	return $arr;
}

/**
 *	整理数组
 *	把多位数据改为一维数组
 *	@param array $arr 多维数组
 *	@param array $yw 一维数据
 *	@return array 一维数组
 */
function arrayfor($arr,&$yw=array()){
	foreach($arr as $k=>$v){
		if(is_array($v)){
			arrayfor($v,$yw);
		}else{
			$yw[] = $v;
		}
	}
	return $yw;
}
/**
 *	数据保存
 *	@param string $name 保存的名字
 *	@param string $data 保存的数据
 */
function save($name,$data){
	$res = fopen($name,'w');
	fwrite($res,$data);
	fclose($res);
	return true;
}

/**
 *	查找的单词和位置
 *	@param string $word
 */
function fwd($word,$file='bwlblog/ThinkPHP'){
	$arr = dir_name($file);
	$arr = arrayfor($arr);

	//echo '<pre>';var_dump($arr);exit;
	
	$str = '';//准备的空字符
	foreach($arr as $v){
		$fp = fopen($v,'r');
		$count = 1;
		while(!feof($fp)){
			$data = fgets($fp,1024);
			$r = strpos($data,$word);
			//var_dump($r);
			if($r!==false){
				$p = $word.'在'.$v.'文件的第'.$count.'行:'.'第'.$r.'个单词:'.'发现了'."<br/>\r\n";
				echo $p;
				$str.=$p;
			}
			++$count;
		}
		fclose($fp);
	}
	//save($word.'.txt',$str);
	save('pp.txt',$str);
	return $str;
}

//test
fwd('要查找的字符串','文件名');
?>

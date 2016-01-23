<?php
/**
 *	定时删除过期文件
 */

define('CACHE_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
define('CACHE_FN', CACHE_ROOT.'cache/');

$timeout = 6*60*60;//单位s

set_time_limit(0);
ini_set('memory_limit','128M');

function gfiletype($fn){
	$f = explode('.', $fn);
	$c = count($f);
	if($c>1){
		$c = $c - 1;
	}else{
		$c = 1;
	}
	return $f[$c];
}

function del_file($dir){
	global $timeout;
	$d = opendir($dir);
	while($s = readdir($d)){
		if('.'== $s || '..' == $s){
		}elseif(is_dir($dir.$s)){
			$fn = $dir.$s.'/';
			del_file($fn);
		}elseif(is_file($dir.$s)){
			$fn = $dir.$s;
			/*if(in_array(gfiletype($s),
				array(
					'js',
					'css',
					'swf',
				))){
				echo $fn, "\t", "文件类型不用检查!!", "\n";
				continue;
			}*/


			
			$ntime = time();
			clearstatcache();
			$ftime = filemtime($fn);
			if(($ftime+$timeout) < $ntime){
				unlink($fn);
				echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '已经删除', "\n";
			}else{
				date_default_timezone_set('PRC');
				echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '未删除', "\n";
			}
		}
	}
}
del_file(CACHE_FN);


echo "\r\n\r\n";
echo "system run time:", date('Y-m-d H:i:s');
?>

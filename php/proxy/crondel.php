<?php
/**
 *	定时删除过期文件
 */
header('Content-type:text/html;charset=utf-8');
date_default_timezone_set('PRC');
define('CACHE_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
//define('CACHE_FN', CACHE_ROOT.'cache/');

//缓存时间(缓存一周)
$timeout = 24*60*60;//单位s

set_time_limit(0);
ini_set('memory_limit', '512M');

//获取文件类型
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

function is_file_list_empty($dir){
	$flist = scandir($dir);
	if(count($flist) >2){
		return false;//有文件
	}
	return true;
}

$protect = array(
	'dir' => array(
		'p',
		'cache'
	),
	'file' => array(
		'.htaccess',
		'config.php',
		'crondel.php',
		'del.sh',
		'FileCache.class.php',
		'FileCache_n.class.php',
		'index.php',
		'proxy.php',
		't.log'
	),
);


function del_file($dir){
	global $timeout, $protect;
	$d = @opendir($dir);


	while($s = @readdir($d)){
		if('.'== $s || '..' == $s){
		}elseif(is_dir($dir.$s)){
			$fn = $dir.$s.'/';
			del_file($fn);

			$bool = is_file_list_empty($fn);

			//var_dump($bool);
			//为空,并且不在保护的文件夹中
			if( $bool && !in_array($s, $prorect['dir'])){
				$rmdir_b = rmdir($fn);
				if($rmdir_b){
					echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '已经删除', "\n<br/>";
				}else{
					echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '删除失败,权限不足!!!', "\n<br/>";
				}
			}
			
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

			if(in_array($s, $protect['file'])){
				continue;
			}

			
			$ntime = time();
			clearstatcache();
			$ftime = filemtime($fn);
			if(($ftime+$timeout) < $ntime){
				unlink($fn);
				echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '已经删除', "\n<br/>";
			}else{	
				echo $fn, "\t", date('Y-m-d H:i:s', $ftime), "\t", '未删除', "\n<br/>";
			}
		}
	}
}
del_file(CACHE_ROOT);
//
//var_dump(is_file_list_empty(CACHE_ROOT));


echo "<hr />";
echo "\r\n\r\n";
echo "system run time:", date('Y-m-d H:i:s');
?>

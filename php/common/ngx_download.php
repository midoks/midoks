<?php

//test.cn/ngx_download.php?fn=hjs.zip

$fName = $_GET['fn'];

header('Content-type: application/octet-stream');
$ua = $_SERVER['HTTP_USER_AGENT'];

if(preg_match('/MSIE/', $ua)){
	$encode_fn = rawurlencode($fName);
	header('Content-Disposition: attachment; filename="'.$encode_fn.'"');
} else if (preg_match('/Firefox/', $ua)){
	header("Content-Disposition: attachment; filename*=\"utf8''".$fName.'"');
} else{	
	header('Content-Disposition: attachment; filename="'.$fName.'"');
}


header("X-Accel-Redirect: /".$fName);


?>
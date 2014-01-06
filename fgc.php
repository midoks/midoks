<?php
/**
 *	@func 妙用PHP file_get_contents
 *	@ref http://www.ietf.org/rfc/rfc1867
 */

//最简单的使用方法
//$res = file_get_contents('http://midoks.cachecha.com');


//POST
//要创建一个上下文
/*$context = stream_context_create(array(
	'http'=> array(
		'method' => 'POST', //使用方法
		'timeout'=> 1, // 设置超时时间
		'header' => 'connection: close'.
			"\n", //头信息
		'content'=>'test=nihao'
	)
));
$res = file_get_contents('http://localhost/hello.php', false, $context);
 */


//POST XML文件
/*$xml = '<xml><info><name>小明</name><age>18</age></info></xml>';
$context = stream_context_create(array(
	'http'=> array(
		'method' => 'POST', //使用方法
		'timeout'=> 1, // 设置超时时间
		'header' => 'connection: close'."\n"
			."Content-type: application/x-www-form-urlencoded\n"
			.'Content-Length:'.strlen($xml)."\r\n", //头信息
		'content'=> $xml
	)
));
$res = file_get_contents('http://localhost/hello.php', false, $context);
*/

//上传文件
/*
$fn  = 'c1.txt';
$pos = dirname(__FILE__).'/'.$fn;
$fc = file_get_contents($pos);
//$bname = basename($fn);
$boundary = substr(md5(rand(0,32000)), 0, 10);

$data .= "--$boundary\n";
$data .= "Content-Disposition: form-data; name=\"file\"\n\n";
$data .= "file\n";


$data .= "--$boundary\n";
$data .= "Content-Disposition: form-data; name=\"file\";filename=\"{$fn}\"\n";
$data .= 'Content-Type: image/gif'."\n";	
$data .= 'Content-Transfer-Encoding: binary'."\n\n";
$data .= $fc."\n";
$data .= "--$boundary--\n";
//echo($data);

$context = stream_context_create(array(
	'http'=> array(
		'method' => 'POST', // 使用方法
		'timeout'=> 10, // 设置超时时间
		'user_agent'=>$_SERVER['HTTP_USER_AGENT'],
		'header' =>"Content-Type: multipart/form-data; boundary={$boundary}", //头信息
		'content'=> $data
	)
));
$res = file_get_contents('http://localhost/hello.php', false, $context);
//$fp = fopen('http://localhost/hello.php', 'rb', false, $context);//这个也能实现
echo $res;
 */

/*$context = stream_context_create(array(
	'http' => 
		array(
			'proxy' => 'tcp://127.0.0.1:8080',
			'request_fulluri' => true
		)
	)
);
$res = file_get_contents('http://localhost/hello.php', false, $context);

echo '<pre>';
echo $res;
 */
?>

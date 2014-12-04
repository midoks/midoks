<?php
/**
 *	@func ZIP算法测试
 */
include('zip.class.php');
define('ABSROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
$zip = new zip();
$ret = $zip->compress(ABSROOT.'/weixin_sdk/', ABSROOT.'test.zip');
?>

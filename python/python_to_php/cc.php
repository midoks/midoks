<?php

//加密
$key  = 'ABCDEFGHIJKLMNOP'; //16位
$vi   = '0102030405060708'; //16位
$str  = '需加密的字符串';
$sign = openssl_encrypt($str, 'AES-128-CBC', $key, OPENSSL_RAW_DATA, $vi);
$sign = base64_encode($sign);
var_dump($sign);
//结果：F8dIExttsNN1WK5BCLHhkRZVwVIoI70W1G+pjPhgNRk=

///////0000000000 解密
$key = 'ABCDEFGHIJKLMNOP'; //16位
$vi  = '0102030405060708'; //16位

$data = 'KeAJB/1sMcGk6HUmKeC5jA==';
$sign = openssl_decrypt(base64_decode($data), 'AES-128-CBC', $key, 1, $vi);
var_dump($sign);die();
//结果：abcd123

?>
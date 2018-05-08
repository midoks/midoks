<?php

//敏感词,过滤程序

define('ABSPATH', str_replace('\\', '/', dirname(__FILE__)).'/' );
include(ABSPATH.'md_censor.php');



if (!file_exists('sss1.txt')){

	$censor_content = file_get_contents('word2.txt');
	$censor_words = explode("\n", $censor_content);
	//var_dump($censor_words);

	$wd_censor = new word_censor();


	$count = count($censor_words);
	for ($pos=0; $pos < $count; $pos++) { 
		$wd_censor->insert($censor_words[$pos]);
	}

	$content = $wd_censor->export();

	file_put_contents('sss1.txt', $content);
} else {

	$censor_content = file_get_contents('sss1.txt');
	$wd_censor = new word_censor();

	$wd_censor->import($censor_content);
}

$time_start = microtime(true);
$bb = $wd_censor->contain(file_get_contents('art.txt'),true);
$time_end = microtime(true);

var_dump($time_end - $time_start);

var_dump($bb);


?>
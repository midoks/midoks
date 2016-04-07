<?php

$fp = fopen('access.log', 'a');

function add($content, $file){
	global $fp;
	fwrite($fp, $content."\n");
}

echo "OK! start\n";

$start_time = time();

for($i=0;$i<10000000;$i++)
{
	if ($i%1000001 == 0){
		echo "{$i}\n";
		
		$end_time = time();

		$work_time = $end_time - $start_time;
		echo "ºÄÊ±:{$work_time}Ãë\n";
		$start_time = $end_time;
	}

	add('192.168.159.1 - - [28/Mar/2016:16:00:52 +0800] "GET /p.php?act=rt&callback=jQuery1701620699632171816_1459151986436&_=1459152048924 HTTP/1.1"'.
		' 200 430 "http://192.168.159.128/p.php" "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.87 Safari/537.36" -', 'access');
}

fclose($fp);


?>
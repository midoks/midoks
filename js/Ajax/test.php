<?php

if(!empty($_POST)){
	$str = json_encode($_POST);
	echo $str;
	save('r.txt',$_SERVER['REMOTE_ADDR'].$str);
}
if(!empty($_GET)){
	$str = json_encode($_GET);
	echo $str;
	save('r.txt',$_SERVER['REMOTE_ADDR'].$str);
}

function save($name,$txt){
	$fn=fopen($name,'ab');
	fwrite($fn,$txt."\r\n");
	fclose($fn);
}	

?>

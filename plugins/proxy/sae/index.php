<?php

$uri = $_SERVER['REQUEST_URI'];

//echo $uri;exit;

include('SaeStorageT.php');
$objSae = new SaeStorageT('midokst');

$dataSae = $objSae->get($uri);

if(!empty($dataSae)){
	echo $dataSae;exit;
}else{
	
	ob_start();
	include('proxy.php');
	$data = ob_get_contents();

	$objSae->save($uri, $data);
}
?>

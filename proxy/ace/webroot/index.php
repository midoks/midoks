<?php
//Alibaba::Xhprof()->start();


$uri = $_SERVER['REQUEST_URI'];
//include('MdAliOSS.php');

//phpinfo();
//$objAli = new MdAliOSS('midoks');
//$dataAli = $objAli->get($uri);


//exit('dd');
//if(!empty($dataAli)){
//	echo $dataAli;exit;
//}else{
//	ob_start();
	include('proxy.php');
//	$data = ob_get_contents();
//	$objAli->save($uri, $data);
//}

//Alibaba::Xhprof()->finish();
?>

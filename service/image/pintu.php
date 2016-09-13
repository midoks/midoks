<?php

/**
 * 拼图服务
 * author midoks
 * 需要 php imageick
 */


//test data
$_REQUEST['pics'][] = "http://thumb1.yokacdn.com/p_700_500/sp1/201609/12/2128300_14736464157lHU.png.jpg";
$_REQUEST['pics'][] = "http://thumb1.yokacdn.com/p_700_500/sp1/201609/12/2128300_1473646417i91K.png.jpg";
$_REQUEST['pics'][] = "http://thumb1.yokacdn.com/p_700_500/sp1/201609/12/2128300_1473646418Cs1L.png.jpg";

if(isset($_REQUEST['pics']) && empty($_REQUEST['pics']) && !is_array($_REQUEST['pics'])){
	return 'error';
}


$nImage = new PintuSrv($_REQUEST['pics']);
$nImage->pImg();



class PintuSrv{

	const QUALITY = 80;

	public function __construct($pics){

		foreach ($pics as $k => $v) {
			$md5_name = md5($v).'.jpg';
			if(!file_exists($md5_name)){
				file_put_contents($md5_name, file_get_contents($v));
			}
		}
	}

	public function pImg(){


	}

	public function getImageSize($image){

	}


	public function __dd(){

	}
}
?>
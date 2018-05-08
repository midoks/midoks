<?php

/**
 * 拼图服务
 * author midoks
 * 需要 php imageick
 */


//test data
//$_REQUEST['pics'][] = "http://bp2.yokacdn.com/attachments/day_160901/210829jr8fziingens9fyy.jpg";
//$_REQUEST['pics'][] = "http://bp2.yokacdn.com/attachments/day_160901/210850zjjufob0rejuma4d.jpg";
//$_REQUEST['pics'][] = "http://bp2.yokacdn.com/attachments/day_160901/210855wkln3o3xwlkdzkxx.jpg";

if(isset($_REQUEST['pics']) || empty($_REQUEST['pics']) || !is_array($_REQUEST['pics'])){
	echo 'error';exit;
}


$nImage = new PintuSrv($_REQUEST['pics']);
$nImage->pImg();

class PintuSrv{

	const QUALITY = 80;

	public $listImagIck = array();

	public function __construct($pics){

		foreach ($pics as $k => $v) {
			$md5_name = md5($v).'.jpg';
			if(!file_exists($md5_name)){
				file_put_contents($md5_name, file_get_contents($v));
			}

			$imagick = new imagick($md5_name);
			$isize 	= $imagick->getImageGeometry();

			$info['ick'] = $imagick;
			$info['w'] = $isize['width'];
			$info['h'] = $isize['height'];
			$this->listImagIck[] = $info;
		}
	}

	public function pImg(){

		$this->sortArrByField($this->listImagIck, 'w');
		$imgList = $this->listImagIck;
		$inum = count($imgList);

		$newImage = new imagick();
		$newImage->newImage(750, 500, 'white');
		$newImage->setimageformat('jpg');
		$newImage->setCompressionQuality(self::QUALITY);

		if($inum == 0){
			return '';
		} else if ($inum == 1){

			$tmp = $imgList[0]['ick'];
			$tmp->cropThumbnailImage(750, 500);
			$tmp->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp, Imagick::COMPOSITE_OVER, 0, 0);

		} else if ($inum == 2){

			$tmp = $imgList[0]['ick'];
			$tmp->cropThumbnailImage(375-1, 500);
			$tmp->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp, Imagick::COMPOSITE_OVER, 0, 0);

			$tmp1 = $imgList[1]['ick'];
			$tmp1->cropThumbnailImage(375, 500);
			$tmp1->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp1, Imagick::COMPOSITE_OVER, 375, 0);

		} else if ($inum == 3){

			$tmp = $imgList[0]['ick'];
			$tmp->cropThumbnailImage(500-1, 500);
			$tmp->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp, Imagick::COMPOSITE_OVER, 0, 0);


			$tmp1 = $imgList[1]['ick'];
			$tmp1->cropThumbnailImage(250, 250-1);
			$tmp1->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp1, Imagick::COMPOSITE_OVER, 500, 0);

			$tmp2 = $imgList[2]['ick'];
			$tmp2->cropThumbnailImage(250, 250);
			$tmp2->setCompressionQuality(self::QUALITY);
			$newImage->compositeImage($tmp2, Imagick::COMPOSITE_OVER, 500, 250);
		}

		$newImage->writeImage('p.jpg');
	}

	//排序,默认从大到小
	public function sortArrByField(&$array, $field, $desc = true){
	  	$fieldArr = array();
	  	foreach ($array as $k => $v) {
	  	  $fieldArr[$k] = $v[$field];
	  	}
	  	$sort = $desc == false ? SORT_ASC : SORT_DESC;
	  	array_multisort($fieldArr, $sort, $array);
	}
}
?>
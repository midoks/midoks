<?php
/**
 *	@func 把JS文件压缩成图片
 *  这样可以下载速度
 *	@need:GD2支持
 */

ini_set('memory_limit', -1);
set_time_limit(0);
 
$fileName = 'PHPjs_0.1.1.js';
/**
 *
 */
function ImgEncode($fn){
	//检测文件是否存在
	if(!file_exists($fn)){
		exit($fn.'文件不存在');
	}
	//文件大小
	$fsize = filesize($fn);
	//计算生成图片的宽度
	$iWidth = ceil(sqrt($fsize/1));
	//生成图片的高度
	$iHeight = $iWidth;
	//生成图片的资源
	$im = imagecreatetruecolor($iWidth, $iHeight);
	$fs = fopen($fn, 'r');//只读打开读取文件内容
	$data = fread($fs, $fsize);//文件内容
	fclose($fs);
	$i = 0;
	for($y = 0; $y < $iHeight; ++$y){
		for($x = 0; $x < $iWidth; ++$x){
			//echo ord($data[$i]);
			$ord = ord($data[$i]);	
			//创建像素点的颜色
			$color = imagecolorallocate($im, $ord, 0, 0);//图像的颜色
			//在图片上以像素点保存数据
			imagesetpixel($im, $x, $y, $color);
			++$i;
		}
	}
	return $im;
}


function ImgDecode($png){
	//检测文件是否存在
	if(!file_exists($png)){
		exit($png.'文件不存在');
	}
	
	$im = imagecreatefrompng($png);//获取图片的资源
	//图片的宽和高
	list($iWidth, $iHeight) = getimagesize($png);
	//$width = imagesx($im);
	//$height = imagesy($im);//这种方法不是很好
	$temp = '';
	$i = 0;
	for($y = 0; $y < $iHeight; ++$y){
		for($x = 0; $x < $iWidth; ++$x){
			$rgb = imagecolorat($im, $x, $y);
			$r = ($rgb >> 16) & 0xFF;
			//$g = ($rgb >> 8) & 0xFF;
			//$b = $rgb & 0xFF;//只取一个值
			$data = chr($r);
			$temp .= $data;
			++$i;
		}
	}
	//print_r($temp);
	return $temp;
}


/**
 *  @func 测试
 */

$im = ImgEncode($fileName);//执行
//header头
//header("Content-Type: image/png");
imagepng($im,'test.png');
//imagedestroy($im);




//2.读取数据
//$text = ImgDecode('test.png');
//file_put_contents('t.txt', $text);
?>
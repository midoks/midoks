<?php

/**
 * @func 图片截取
 * @param string  图片地址
 * @param string $new_addr 新图片地址
 * @param int $x1 左边截取位置(x的位置)
 * @param int $y1 左边截取位置(y的位置)
 * @param int $x2 右边截取位置(x的位置)
 * @param int $y2 右边截取位置(y的位置)
 * @return string 返回截取的图片位置
 */
function ImageSub($addr, $new_addr,$x1, $y1, $x2, $y2){
	$name = strtolower(pathinfo($addr, PATHINFO_EXTENSION));
	if('jpg' == $name){$name = 'jpeg';}
	$createFrom = 'imagecreatefrom'.$name;
	$imageTo = 'image'.$name;

	$s_im = $createFrom($addr);
	
	$width=$x2-$x1;//截取后图片的宽度
	$height = (int)($y2-$y1);//截取后图片的高度
	//$h = substr($height,0,1)=='-' ? -$height : $height;
	if(substr($height,0,1)=='-'){
		$n_im = imagecreatetruecolor($width,-$height);//建立缩放后的图片资源
		imagecopyresampled($n_im, $s_im, 0, 0, $x1, $y1+$height, $width,-$height, $width, -$height);//截取
	}else{
		$n_im = imagecreatetruecolor($width, $height);//建立缩放后的图片资源
		imagecopyresampled($n_im,$s_im, 0, 0, $x1, $y1, $width, $height, $width, $height);//截取
	}
	$imageTo($n_im, $new_addr);//生成新图片
	imagedestroy($s_im);imagedestroy($n_im);//销毁资源
	return $addr;
}
?>

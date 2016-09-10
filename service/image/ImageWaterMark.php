<?php
/**
 *  @func 为图片添加循环水印
 *  @param 水印可以是文字或图片
 */
class ImageWaterMark{
    /**
     * 取得图像信息
     * @static
     * @access public
     * @param string $image 图像文件名
     * @return mixed
     */
 
    public static function getImageInfo($img) {
        $imageInfo = getimagesize($img);
        if ($imageInfo !== false) {
            $imageType = strtolower(substr(image_type_to_extension($imageInfo[2]), 1));
            $imageSize = filesize($img);
            $info = array(
                "width" => $imageInfo[0],
                "height" => $imageInfo[1],
                "type" => $imageType,
                "size" => $imageSize,
                "mime" => $imageInfo['mime']
            );
            return $info;
        } else {
            return false;
        }
    }
 
    /**
     *  @func 添加图片的文字水印
     *  @param string $source   要添加水印的图片路径
     *  @param string $water    水印图片(png)
     *  @param string $savename 保存名
     */
    public static function image($source, $water, $savename){
         //检查文件是否存在
        if (!file_exists($source) || !file_exists($water)){
            return false;
        }
 
        //图片信息
        $sInfo = self::getImageInfo($source);
        $wInfo = self::getImageInfo($water);
 
        //如果图片小于水印图片，不生成图片
        if ($sInfo['width'] < $wInfo['width'] || $sInfo['height'] < $wInfo['height']){
            return false;
        }
 
        //建立图像
        $sCreateFun = 'imagecreatefrom'.$sInfo['type'];
        $sImage = $sCreateFun($source);
        $wCreateFun = 'imagecreatefrom'.$wInfo['type'];
        $wImage = $wCreateFun($water);
 
        //设定图像的混色模式
        imagealphablending($wImage, true);
        //高 num
        $height_num = ceil($sInfo['height']/$wInfo['height']);
        //宽 num
        $width_num = ceil($sInfo['width']/$wInfo['width']);
         
        for($i=0; $i<$height_num; ++$i){
            $posY = $wInfo['height']*$i;
            for($y=0; $y<$width_num; ++$y){
                $posX =  $wInfo["width"]*$y;
                imagecopy($sImage, $wImage, $posX, $posY, 0, 0, $wInfo['width'], $wInfo['height']);
            }
        }
        //输出图像
        $ImageFun = 'Image' . $sInfo['type'];
        //如果没有给出保存文件名，默认为原图像名
        if (!$savename) {
            $savename = $source;
            @unlink($source);
        }
        //保存图像
        $ImageFun($sImage, $savename);
        imagedestroy($sImage);
    }
}
?>
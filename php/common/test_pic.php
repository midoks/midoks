<?php 
$GIF_DATA = array(
		chr(0x47),chr(0x49),chr(0x46),chr(0x38),chr(0x39),chr(0x61),
		chr(0x01),chr(0x00),chr(0x01),chr(0x00),chr(0x80),chr(0xff),
		chr(0x00),chr(0xff),chr(0xff),chr(0xff),chr(0x00),chr(0x00),
		chr(0x00),chr(0x2c),chr(0x00),chr(0x00),chr(0x00),chr(0x00),
		chr(0x01),chr(0x00),chr(0x01),chr(0x00),chr(0x00),chr(0x02),
		chr(0x02),chr(0x44),chr(0x01),chr(0x00),chr(0x3b)
);
header('Content-type: image/jpeg');
echo join($GIF_DATA);
exit ;
?>
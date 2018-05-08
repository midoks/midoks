<?php 

$html = "<table><tr><td>id</td></tr><tr><th>1</th></tr></table>";
header('Content-Transfer-Encoding: gbk');
header('Content-Type: application/vnd.ms-excel;');
header("Content-type: application/x-msexcel");
header(iconv('UTF-8', 'GBK//IGNORE', 'Content-Disposition: attachment; filename="用户详细信息_' . date('Ymd') . '.xls"'));
echo iconv('UTF-8', 'GBK//IGNORE', $html);
?>
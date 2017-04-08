<?php 


include('common.php');


var_dump($dbModel);

$d = $dbModel->getOne('select count(*) as num from `wiki_advertisement`');

var_dump($d);


?>
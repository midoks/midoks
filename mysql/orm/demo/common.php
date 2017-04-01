<?php 


$config = include('databases.php');

include('../ORM/db.class.php');

$dbModel = DB::getInstance();

$dbModel->injection($config['default']);
//var_dump($dbModel);


?>
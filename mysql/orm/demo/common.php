<?php 


$config = include('databases.php');

include('../ORM/db.class.php');

$dbModel = new DB();

$dbModel->injection($config['default']);
//var_dump($dbModel);


?>
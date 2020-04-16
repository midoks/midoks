<?php

class Demo {

    public $links = NULL;

    //链接数据库
    public function __construct() {
        $this->demo1();
    }

    public function demo1() {
        $dsn      = 'mysql:dbname=dd;host=127.0.0.1';
        $user     = 'root';
        $password = 'root';
        try {
            $params = [
                PDO::MYSQL_ATTR_INIT_COMMAND => "set names utf8", //设置编码
                PDO::ATTR_PERSISTENT         => false,
            ];
            $db = new PDO($dsn, $user, $password, $params);

        } catch (PDOException $e) {
            echo $e->getMessage();
            exit;
        }

    }

}

$obj = new Demo;

?>
<?php

namespace App\Utils;

/**
 * ElasticSearch class
 * 参考文档:https://www.elastic.co/guide/en/elasticsearch/reference/current/docs.html
 */

class ElasticSearch {

    public $index;

    private static $instance = NULL;

    private function __construct() {}

    //单利模式
    public static function getInstance() {
        if (self::$instance) {
            self::$instance = new self;
        }
        return self::$instance;
    }

    //设置服务器地址
    public function setServer($server = 'http://localhost:9200') {
        $this->server = $server;
    }

    //统一调用
    public function call($path, $http = array()) {
        if (!$this->index) {
            throw new Exception('$this->index needs a value');
        }

        $url      = $this->server . '/' . $this->index . '/' . $path;
        $stream   = stream_context_create(array('http' => $http));
        $ret_data = file_get_contents($url, NULL, $stream);

        return json_decode($ret_data, true);
    }

    //curl -X PUT http://localhost:9200/{INDEX}/
    //创建索引
    public function create() {
        return $this->call(NULL, array('method' => 'PUT'));
    }

    //curl -X DELETE http://localhost:9200/{INDEX}/
    //删除缩影
    public function drop() {
        return $this->call(NULL, array('method' => 'DELETE'));
    }

    //curl -X GET http://localhost:9200/{INDEX}/{TYPE}/_count -d {matchAll:{}}
    public function count($type) {
        return $this->call($type . '/_count');
    }

    //curl -X PUT http://localhost:9200/{INDEX}/{TYPE}/_mapping -d ...
    public function map($type, $data) {

        $args = array('method' => 'PUT',
            'content'              => $data,
            'header'               => 'Content-Type: application/x-www-form-urlencoded',
            'Content-Length'       => strlen($data));

        return $this->call($type . '/_mapping', $args);
    }

    //curl -X PUT http://localhost:9200/{INDEX}/{TYPE}/{ID} -d ...
    //添加数据
    public function add($type, $data, $id_pre = 'uu_') {
        $id   = $id_pre . microtime(true);
        $args = array('method' => 'PUT',
            'content'              => $data,
            'header'               => 'Content-Type: application/x-www-form-urlencoded',
            'Content-Length'       => strlen($data));

        return $this->call($type . '/' . $id, $args);
    }

    //curl -XDELETE 'http://localhost:9200/twitter/tweet/1'
    //删除数据
    public function delete($type, $id) {
        $args = array('method' => 'DELETE');
        return $this->call($type . '/' . $id, $args);
    }

    //更新数据
    public function update($type, $id, $data) {
        $args = array('method' => 'PUT',
            'content'              => $data,
            'header'               => 'Content-Type: application/x-www-form-urlencoded',
            'Content-Length'       => strlen($data));
        return $this->call($type . '/' . $id, $args);
    }

    //curl -X GET http://localhost:9200/{INDEX}/{TYPE}/_search?q= ...
    //url查询
    public function query($type, $q) {
        return $this->call($type . '/_search?' . http_build_query(array('q' => $q)));
    }

    //post data 查询
    public function search($type, $data) {
        $args = array('method' => 'PUT',
            'content'              => $data,
            'header'               => 'Content-Type: application/x-www-form-urlencoded',
            'Content-Length'       => strlen($data));

        return $this->call($type . '/_search', $args);
    }

}

?>
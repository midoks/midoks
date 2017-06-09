<?php

class Curl {

	public $curl = NULL;
	private $post = array();

	/**
	 * 构造函数
	 * @param $url string HTTP地址
	 */
	public function __construct($url = ''){
		$this->curl = curl_init($url);
		curl_setopt($this->curl, CURLOPT_RETURNTRANSFER, TRUE);
	}

	/**
	 * 设置请求地址
	 * @param $url string HTTP地址
	 */
	public function setUrl($url){
		curl_setopt($this->curl, CURLOPT_URL, $url);
		return $this;
	}

	/**
	 * POST数据
	 * @param $data mixed
	 */
	public function setPostData($data){
		$this->post = array_merge($data, $this->post);
        return $this;
	}

	/**
	 * 文件上传
	 * @param $file string 本地地址(绝对地址)
	 * @param $sign string 文件标示
	 * @return $this object
	 */
	public function uploadFile($file, $sign = 'file'){
		if (class_exists('\CURLFile')) {
            $data = array( $sign => new \CURLFile(realpath($file)) );
        } else {
            $data = array( $sign => "@".$file );
        }
        $this->post = array_merge($data, $this->post);
        return $this;
	}

	/**
	 * 设置curl值
	 * @param $const_curl_var CURLOPT KEY值
	 * @param $value VALUE值
	 */
	public function set($const_curl_var, $value){
		curl_setopt($this->curl, $const_curl_var, $value);
		return $this;
	}

	/**
	 * 发送地址
	 * @return $mixed
	 */
	public function send(){
		if (!empty($this->post)){
			 curl_setopt($this->curl, CURLOPT_POST, 1);
			 curl_setopt($this->curl, CURLOPT_POSTFIELDS, $this->post);
		}


        $output = curl_exec($this->curl);
        curl_close($this->curl);
        return $output;
	}

	public function __destruct(){
		$this->curl = NULL;
		$this->post = NULL;
	}
}

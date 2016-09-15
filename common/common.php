<?php
/*-------------------------
	@func $ 时间记录点 $
---------------------------*/
function G($start, $end='', $dec=3) {
    static $_info = array();
    if(!empty($end)) { // 统计时间
        if(!isset($_info[$end])) {
            $_info[$end] = microtime(TRUE);
        }
        return number_format(($_info[$end]-$_info[$start]),$dec);
    }else{ // 记录时间
        $_info[$start] = microtime(TRUE);
    }
}
//static $baseUsage = memory_get_usage();
/*-------------------------
	@func $ 内存记录点 $
---------------------------*/
function M( $start , $end = '' , $dec = 4){
	static $_memory =  array();
	if(!empty($end)){
		if(!isset($memory[$end])){
			$_info[$end] = memory_get_usage();
		}
		return number_format( ($_memory[$end] - $_memory[$start]) , $dec);
	}else{
		$_memory[$start] = memory_get_usage();
	}
}

/*----------------------------
	@func 			$ 时间和内存记录点 $
	@return array 	$ 返回数组 $
------------------------------*/
function R($start,$end='',$dec=3) {
    static $_info = array();
    if(!empty($end)) { // 统计时间和内存
        if(!isset($_info['time'][$end]) && ! isset($_info['mem'][$end])) {
            $_info['time'][$end]   =  microtime(TRUE);
			$_info['mem'][$end] = memory_get_usage();
        }
        return array(
			number_format(($_info['time'][$end]-$_info['time'][$start]),$dec),
			number_format(($_info['mem'][$end]-$_info['mem'][$start]),$dec)
		);
    }else{ // 记录时间内存
        $_info['time'][$start]  =  microtime(TRUE);
		$_info['mem'][$start] = memory_get_usage();
    }
}


/*----------------------------------------------------------------------------------
	@return $ 返回一个日标准的日期 exp[1990年8月9日 12:59:20] $	
	@type $ 输入一个时间的类型mysql[数据记录的时间] file[文件的时间名] time[时间] $
------------------------------------------------------------------------------------*/	
function DateTime( $type = '' ){
	switch( $type ){
		case '':
			$DateTime = date( 'Y-m-d H:i:s' , time() );break;
		case 'file';
			$DateTime = date( 'Y-m-d' , time() );break;
		case 'time';
			$DateTime = date( 'H:i:s' , time() );break;
		default:
			$DateTime = date( 'H:i:s' , time() );
	}
	return $DateTime;
}


/*---------------------
	获取客服端IP地址
-----------------------*/
function get_client_ip(){
	static $ip = null;
	if($ip != null) return $ip;
	if( isset( $_SERVER['HTTP_X_FORWARDED_FOR'] ) ){
		$arr = explode( ',' ,$_SERVER['HTTP_X_FORWARDED_FOR'] );
		$pos = array_search( 'unknown' , $arr );
		if( false !=$pos ) unset($arr[$pos]);
		$ip = trim( $arr[0] );
	}elseif( isset( $_SERVER['HTTP_CLIENT_IP'] ) ){
		$ip = $_SERVER['HTTP_CLIENT_IP'];
	}elseif( isset($_SERVER['REMOTE_ADDR'] ) ){
		$ip = $_SERVER['REMOTE_ADDR'];
	}
	//检查IP地址的合法性
	$ip = (false!==ip2long($ip)) ? $ip : '0,0,0,0';
}

//写入信息
function Write($fn, $text){
	$fp = fopen($fn, 'ab');
	fwrite($fp, $text."\n");
	fclose($fp);
}



// 读取备份文件中,所有目录
function src_dir_file($name){

	$name = rtrim($name,'/').'/';
	$fp = opendir($name);
	$arr = array();
	while($n = readdir($fp))
		if($n=='.'|| $n=='..'){
		}else if(is_dir($name.$n)){
			$arr[]= src_dir_file($name.$n);
		}else if(is_file($name.$n)){
			$arr[] = $name.$n;
		}
	closedir($fp);
	return $arr;

}

/**
 * 本地文件上传
 * @param $url_filename 本地文件地址
 */
function file_upload($url_filename){

	if (class_exists('\CURLFile')) {
        $files_data = array('uid'=>$uid,'Filedata'=>new \CURLFile(realpath($url_filename)));
    } else {
        $files_data = array('uid'=>$uid,'Filedata'=>"@".$url_filename);
    }

    $api_url = 'http://url';
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, TRUE);
    curl_setopt($ch, CURLOPT_POST, TRUE);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $files_data);
    curl_setopt($ch, CURLOPT_URL, $api_url);
    $ret = curl_exec($ch);
    curl_close($ch);
    return $ret;
}

/**
 *	多位字段排序
 *	@param &$array 数组
 *	@param $field 字段
 *	@param $desc 排序
 */
function sortArrByField(&$array, $field, $desc = false){
  	$fieldArr = array();
  	foreach ($array as $k => $v) {
  	  $fieldArr[$k] = $v[$field];
  	}
  	$sort = $desc == false ? SORT_ASC : SORT_DESC;
  	array_multisort($fieldArr, $sort, $array);
}


/**
 * 保证文件存在
 * @parma 目录路径
 */
function mkdir_p($absdir){
	$absdir = str_replace('\\', '/', $absdir);
	$absdir = rtrim($absdir, '/');
	if(file_exists($absdir)){
		return true;
	}
	$pre_dir = dirname($absdir);
	if(!file_exists($pre_dir)){
		mkdir_p($pre_dir);
	}
	return mkdir($absdir);
}

function is_email($user_email){
    $chars = "/^([a-z0-9+_]|\\-|\\.)+@(([a-z0-9_]|\\-)+\\.)+[a-z]{2,6}\$/i";
    if (strpos($user_email, '@') !== false && strpos($user_email, '.') !== false) {
        if (preg_match($chars, $user_email)){
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

/**
 * 检查是否为一个合法的时间格式
 *
 * @access  public
 * @param   string  $time
 * @return  void
 */
function is_time($time){
    $pattern = '/[\d]{4}-[\d]{1,2}-[\d]{1,2}\s[\d]{1,2}:[\d]{1,2}:[\d]{1,2}/';
    return preg_match($pattern, $time);
}

/**
 * url编码
 */
function base64url_encode($data, $pad = null){
	$data = str_replace(array('+','/'), array('-','_'), base64_encode($data));
	if(!$pad){
		$data = rtrim($data, '=');
	}
	return $data;
}

//url解码
function base64url_decode($data){
	return base64_decode(str_replace(array('-','_'), array('+','/'), $data));
}








?>

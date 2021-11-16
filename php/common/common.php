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

//匹配图片
function get_image_list($content) {
    $imageList = array();
    if (stripos($content, '<img') !== false) {
        preg_match_all('/<img[^<>]*src=[\'|\"]*([^\'\"\s>]+)[\'|\"](.*)[^<]+>/i',$content, $oriImageList);
        $imgList = $oriImageList[1];
        foreach ($imgList as $img) {
            $imageList[] = $img;
        }
    }
    return $imageList;
}



/**
 * 全/半角转换函数
 *
 * @param string 字符串
 * @param int $type 0,半角到全角；1，全角到半角
 * @return string
 */
function zh_en_change($str,$type) {
	$ZH_ARR = Array(
	'０' , '１' , '２' , '３' , '４' ,
	'５' , '６' , '７' , '８' , '９' ,
	'Ａ' , 'Ｂ' , 'Ｃ' , 'Ｄ' , 'Ｅ' ,
	'Ｆ' , 'Ｇ' , 'Ｈ' , 'Ｉ' , 'Ｊ' ,
	'Ｋ' , 'Ｌ' , 'Ｍ' , 'Ｎ' , 'Ｏ' ,
	'Ｐ' , 'Ｑ' , 'Ｒ' , 'Ｓ' , 'Ｔ' ,
	'Ｕ' , 'Ｖ' , 'Ｗ' , 'Ｘ' , 'Ｙ' ,
	'Ｚ' , 'ａ' , 'ｂ' , 'ｃ' , 'ｄ' ,
	'ｅ' , 'ｆ' , 'ｇ' , 'ｈ' , 'ｉ' ,
	'ｊ' , 'ｋ' , 'ｌ' , 'ｍ' , 'ｎ' ,
	'ｏ' , 'ｐ' , 'ｑ' , 'ｒ' , 'ｓ' ,
	'ｔ' , 'ｕ' , 'ｖ' , 'ｗ' , 'ｘ' ,
	'ｙ' , 'ｚ' , '－' , '　'  , '：' ,
	'．' , '，' , '／' , '％' , '＃' ,
	'！' , '＠' , '＆' , '（' , '）' ,
	'＜' , '＞' , '＂' , '＇' , '？' ,
	'［' , '］' , '｛' , '｝' , '＼' ,
	'｜' , '＋' , '＝' , '＿' , '＾' ,
	'￥' , '￣' , '｀'
	);
	$EN_ARR = Array( //半角
	'0', '1', '2', '3', '4',
	'5', '6', '7', '8', '9',
	'A', 'B', 'C', 'D', 'E',
	'F', 'G', 'H', 'I', 'J',
	'K', 'L', 'M', 'N', 'O',
	'P', 'Q', 'R', 'S', 'T',
	'U', 'V', 'W', 'X', 'Y',
	'Z', 'a', 'b', 'c', 'd',
	'e', 'f', 'g', 'h', 'i',
	'j', 'k', 'l', 'm', 'n',
	'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x',
	'y', 'z', '-', ' ', ':',
	'.', ',', '/', '%', '#',
	'!', '@', '&', '(', ')',
	'<', '>', '"', '\'','?',
	'[', ']', '{', '}', '\\',
	'|', '+', '=', '_', '^',
	'$', '~', '`'
	);
	if($type==0) {
		return str_replace($EN_ARR,$ZH_ARR,$str);  //半角到全角
	}else {
		return str_replace($ZH_ARR,$EN_ARR,$str);  //全角到半角
	}

}

 /**
 * @func 数据对齐(midoks) 2017-1-12
 * @param $data 数据
 * @param $width 对齐宽度 默认 3
 */
function dataAlignment($data, $width = 3){
    if(is_array($data)){
        $t_num = count($data);
        $n = floor($t_num/$width);
        $t_data = array();
        if ( $n > 0 ){
            $t_data = array_slice($data, 0, $n*$width);
        }
        return $t_data;
    } else {
        return $data;
    }
}


/**
 * 数据库分页折半算法
 * @param $total_num int 总共多少数据
 * @param $page_now int 当前第几页
 * @param $page_size int 每页多少数据(默认10)
 * @param $sort_id  string 排序的字段
 * @param $now_is_desc boolean 当前排序规则 true->desc|false->asc(默认true)
 */
function table_page_sort_algorithm($total_sum, $page_now, $page_size = 10, $sort_id = 'id', $now_is_desc = true) {

	$page_sum = ceil($total_sum / $page_size);
	
	if ($page_now > $page_sum){
		$page_now = $page_sum;
	}

	$last_page_size = $total_sum % $page_size;
	$psize = ($page_sum == $page_now) && $last_page_size ? $last_page_size : $page_size;

	$realpage = $page_sum - $page_now;
	$resort = false;
	$start_limit = ($page_now-1) * $page_size;
	if ( $page_now > ($page_sum/2) && $page_now > 2 ) {
		if ($last_page_size) {
	        $start_limit = max(0, ($realpage - 1) * $page_size + $last_page_size);
	    } else {
	        $start_limit = max(0, $realpage * $page_size + $last_page_size);
	    }

	    if ($now_is_desc){
	    	$sql = " order by {$sort_id} asc ";
		} else {
			$sql = " order by {$sort_id} desc ";
		}

	    $sql .= " limit $start_limit, $psize";
	    $resort = true;
	} else {

		if ($now_is_desc){
			$sql = " order by {$sort_id} desc ";
		} else {
			$sql = " order by {$sort_id} asc ";
		}

		$sql .= " limit $start_limit, $page_size";
	}

	$ret =  array('sql' => $sql,'resort' => $resort);
	return $ret;
}


function is_spider(){
	$spider_list = array (
		'googlebot' 	=> '谷歌',
		'mediapartners-google' => 'Google Adsense',
		'baiduspider' 	=> '百度',
		'bingbot' 		=> '必应',
		'slurp' 		=> '雅虎',
		'Sogou' 		=> '搜狗',
		'sosospider' 	=> '腾讯SOSO',
		'ia_archiver' 	=> 'Alexa',
		'iaarchiver' 	=> 'Alexa',
		'yodaobot' 		=> 'Yodao',
		'sohu-search' 	=> '搜狐',
		'msnbot' 		=> 'MSN',
		'360Spider'		=> '360',
		'DNSPod'		=> 'DNSPod',
		'JianKongBao'	=> '监控宝',
		'YYSpider' 		=> '云云搜索',
	);

	$user_agent = $_SERVER['HTTP_USER_AGENT'];

	$info = array(
		'is_spider' 	=> false,
		'spider_name' 	=> ''
	);

	foreach($spider_list as $k=>$v){
		if((preg_match('/'.$k.'/i', $user_agent))){
			$info['spider_name'] = $k;
			$info['is_spider'] = true;
			break;
		}
	}
	return $info;
}


function userReplace($userName){
	preg_match_all("/[\x{4e00}-\x{9fa5}A-Za-z0-9._]+/u", $userName, $match);
	return $match;
}


/**
 * 根据起点坐标和终点坐标测距离
 * @param  [array]   $from  [起点坐标(经纬度),例如:array(118.012951,36.810024)]
 * @param  [array]   $to    [终点坐标(经纬度)]
 * @param  [bool]    $km        是否以公里为单位 false:米 true:公里(千米)
 * @param  [int]     $decimal   精度 保留小数位数
 * @return [string]  距离数值
 */
function getDistance($from, $to, $km = true, $decimal=2){
    sort($from);
    sort($to);
    $EARTH_RADIUS = 6370.996; // 地球半径系数
    $distance = $EARTH_RADIUS*2*asin(sqrt(pow(sin( ($from[0]*pi()/180-$to[0]*pi()/180)/2),2)+cos($from[0]*pi()/180)*cos($to[0]*pi()/180)* pow(sin( ($from[1]*pi()/180-$to[1]*pi()/180)/2),2)))*1000;
    
    if($km){
        $distance = $distance / 1000;
    }
    return round($distance, $decimal);
}


/**
 * 查找前缀
 **/
function find_prefix($str, $find) {
    $ss = strlen($str);
    $ff = strlen($find);

    if ($ss != $ff){
        return false
    }

    if ($ss < $ff){
        return false
    }

    $prefix = substr($str, 0, $ff);
    if ($prefix == $find){
        return true;
    }
    return false
}


?>

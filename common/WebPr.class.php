<?php
/*--------------------------
 *	网站的PR值获取
 *	兼容了Sae服务的应用
 *	兼容了Bae服务的应用
 ---------------------------*/
class WebPr{

	public $url;
	
	/**
	 *	@param $name $ 初始化域名 $
	 */
	public function __construct($name){
		$this->url = array(
			'pr'=>'http://toolbarqueries.google.com/'.
				'tbr?client=navclient-auto&features=Rank&q=info:'.$name.'&ch='.$this->GooglePr_64($name),
			'sr'=>'http://rank.ie.sogou.com/sogourank.php?ur=http://'.$name.'/'
		);//pr sr 网址
	}

	/**
	 * @param $ 在64位服务器下不变化 $
	 * @param $int $ 整数值 $
	 */
	public function GooglePr_64($url){        		
		$SEED = "Mining PageRank is AGAINST GOOGLE'S TERMS OF SERVICE. Yes, I'm talking to you, scammer.";
		$Result = 0x01020345;
		for( $i=0; $i<strlen( $url ); ++$i){
			$Result ^= ord( $SEED{ $i%87 } ) ^ ord( $url{ $i } );            
            $Result = $Result & 0xFFFFFFFF;
            echo '>>:', ((($Result >> 23) & 0x1FF) & 0xFFFFFFFF)  , '<<:', (($Result << 9) & 0xFFFFFFFF);
			$Result = ((($Result >> 23) & 0x1FF)&0xFFFFFFFF) | (($Result << 9)&0xFFFFFFFF);
			var_dump($Result);
		}	
		return sprintf( "8%x" , $Result);
	}


	/**
	 *	google pr 专用的
	 *	@param $url $ 对URL地址加密 $
	 *	@return $ 返回加密结果 $
	 */
	public function GooglePr($url){
		$SEED = "Mining PageRank is AGAINST GOOGLE'S TERMS OF SERVICE. Yes, I'm talking to you, scammer.";
		$Result = 0x01020345;
		for( $i=0; $i<strlen( $url ); ++$i){
			$Result ^= ord( $SEED{ $i%87 } ) ^ ord( $url{ $i } );
			$Result = (($Result >> 23) & 0x1FF) | $Result << 9;	
		}	
		return sprintf( "8%x" , $Result);
	}

	/**
	 *	@return $ 获取资源 $
	 *	@param  $url $ 网站地址 $
	 */
	public function OpenUrl($url){
		//初始化连接
		$go = curl_init();
		//设置URL地址
		curl_setopt($go , CURLOPT_URL , $url);
		curl_setopt($go , CURLOPT_HEADER , 0);
		//设置是否可以跳转
      	curl_setopt($go , CURLOPT_FOLLOWLOCATION , 1);
		//设置跳转的次数
		curl_setopt($go , CURLOPT_MAXREDIRS , 30);
		curl_setopt($go , CURLOPT_USERAGENT , $_SERVER['HTTP_USER_AGENT']);
		//头文件
		curl_setopt($go , CURLOPT_HEADER , 0 );
		//返回数据流
		curl_setopt($go , CURLOPT_RETURNTRANSFER , 1);
		//Cookie 文件的读取
		//curl_setopt($go , CURLOPT_COOKIEJAR , $cookie_file);
		//运行的超时时间
		$data = curl_exec( $go );
		//传输中的信息
		//$ConnectInfo = curl_getinfo( $go );	
		curl_close( $go );
		//$data = GUNzip( $data );	   	
		return $data;		
	}
  
  	public function BaeOpenUrl($url){
    	require_once "BaeFetchUrl.class.php";
      	$fetch = new BaeFetchUrl();
      	$fetch->setOptionMaxRedirs(5);
      	$fetch->setOptionFollowLocation(true);
      	$fetch->get($url);
        $content= $fetch->getResponseBody();
      	return $content;
    }
	
	/**
	 *	@return $ 返回数据结果 $
	 */
	public function GetData(){
		var_dump($this);
		$data = '';
		foreach($this->url as $k=>$v){
			$webData = $this->OpenUrl($v);
			//$webData = file_get_contents($v);
			$num = substr(trim($webData),-1);
			if($num){
				$data[$k] = $num;
			}else{
				$data[$k] = '0';
			}
		}
		return $data;
	}

}


//test

//$o = new WebPr('www.zhugao.net');
//var_dump($o->GetData());
var_dump(decbin(1082187712));
var_dump(decbin(1082187712<<1));
var_dump(decbin((1082187712<<1) & 0xFFFFFFFF));
var_dump(1082187712<<1);

?>

<?php

class RedisConnection{

	public static $linkID = NULL;

	function __construct(){
		if( self::$linkID == NULL ){

			self::$linkID = new Redis();
			self::$linkID->connect('127.0.0.1', 6379);
		}
	}

	function getLinkID(){
		return self::$linkID;
	}

}


//var_dump($_SERVER);
//var_dump(__FILE__);
//echo ($_SERVER["SCRIPT_FILENAME"]), str_replace('\\', '/', __FILE__);

if($_SERVER["SCRIPT_FILENAME"] == str_replace('\\', '/', __FILE__) ){

	$r = new RedisConnection();

	$r = new RedisConnection();


	var_dump($r::$linkID);

	echo "ok";

}

?>
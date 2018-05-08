<?php
/**
 * 敏感词过滤
 * 参考:	http://blog.11034.org/2012-07/trie_in_php.html
 *   |-	http://blog.csdn.net/woshiaotian/article/details/10047675
 */

class word_censor{

	public $tree = array();
	
	public function __construct(){

	}


	/**
	 * 获取字符数组
	 * @param string $utf8_str utf8字符串数组
	 * @return array
	 */
	public function get_chars($utf8_str){
		$s = $utf8_str;
		$len = strlen($s);
		if($len == 0) return array();
		$chars = array();
		for($i = 0;$i < $len;$i++){
			$c = $s[$i];
			$n = ord($c);
			if(($n >> 7) == 0){		//0xxx xxxx, asci, single
				$chars[] = $c;
			} else if(($n >> 4) == 15){ 	//1111 xxxx, first in four char
				if($i < $len - 3){
					$chars[] = $c.$s[$i + 1].$s[$i + 2].$s[$i + 3];
					$i += 3;
				}
			} else if(($n >> 5) == 7){ 	//111x xxxx, first in three char
				if($i < $len - 2){
					$chars[] = $c.$s[$i + 1].$s[$i + 2];
					$i += 2;
				}
			} else if(($n >> 6) == 3){ 	//11xx xxxx, first in two char
				if($i < $len - 1){
					$chars[] = $c.$s[$i + 1];
					$i++;
				}
			}
		}
		return $chars;
	}


	public function insert($utf8_str){
		$chars = $this->get_chars($utf8_str);
		$chars[] = null;	//串结尾字符
		$count = count($chars);
		$T = &$this->tree;
		for($i = 0;$i < $count;$i++){
			$c = $chars[$i];
			if(!array_key_exists($c, $T)){
				$T[$c] = array();	//插入新字符，关联数组
			}
			$T = &$T[$c];
		}
	}

	public function remove($utf8_str){
		$chars = $this->get_chars($utf8_str);
		$chars[] = null;
		if($this->_find($chars)){	//先保证此串在树中
			$chars[] = null;
			$count = count($chars);
			$T = &$this->tree;
			for($i = 0;$i < $count;$i++){
				$c = $chars[$i];
				if(count($T[$c]) == 1){		//表明仅有此串
					unset($T[$c]);
					return;
				}
				$T = &$T[$c];
			}
		}
	}

	private function _find(&$chars){
		$count = count($chars);
		$T = &$this->tree;
		for($i = 0;$i < $count;$i++){
			$c = $chars[$i];
			if(!array_key_exists($c, $T)){
				return false;
			}
			$T = &$T[$c];
		}
		return true;
	}

	public function contain($utf8_str, $do_count = false){
		$chars = $this->get_chars($utf8_str);
		$chars[] = null;
		$len = count($chars);

		$Tree = $this->tree;
		$count = 0;
		for($i = 0;$i < $len;$i++){
			$c = $chars[$i];
			if(array_key_exists($c, $Tree)){	//起始字符匹配
				$T = &$Tree[$c];
				for($j = $i + 1;$j < $len;$j++){
					$c = $chars[$j];
					if(array_key_exists(null, $T)){
						if($do_count){
							$count++;
						} else {
							return true;
						}
					}

					if(!array_key_exists($c, $T)){
						break;
					}
					$T = &$T[$c];
				}
			}
		}
		if($do_count){
			return $count;
		} else {
			return false;
		}
	}

	public function contain_all($str_array){
		foreach($str_array as $str){
			if($this->contain($str)){
				return true;
			}
		}
		return false;
	}
 
	public function export(){
		return serialize($this->tree);
	}
 
	public function import($str){
		$this->tree = unserialize($str);
	}
 
 
}




?>
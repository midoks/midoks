<?php
/**
 * DHT协议相关
 */

class bencode{

	//编码方法
	public function en($data){
		if(is_array($data) && (isset($data[0]) || empty($data))){
			return $this->en_list($data);
		}else if (is_array($data)){
			return $this->en_dict($data);
		}else if (is_integer($data || is_float($data) ) ){
			$data = sprintf("%.0f", round($data, 0));
			return $this->en_int($data);
		}else{
			return $this->en_string($data);
		}
	}


	private function en_int($data){
		return sprintf("i%.0fe", $data);
	}

	private function en_string($data){
		return sprintf("%d:%s", strlen($data), $data);
	}

	private function en_list($data){
		$list = '';
		foreach($data as $k=>$v){
			$list .= $this->en($v);
		}
		return "l{$list}e";
	}

	private function en_dict($data){
		ksort($data);
		$dict = '';
		foreach($data as $k=>$v){
			$dict .= $this->en_string($k).$this->en($v);
		}
		return "d{$dict}e";
	}

	//解码方式
	public function de($data){
		$this->source = $data;
		$this->offset = 0;
		$this->length = strlen($data);


		$de = $this->do_de();
		
		return $de;
	}


	private function do_de(){
		switch($this->get_char()){
			case 'i': 	++$this->offset;return $this->de_int();
			case 'l':	++$this->offset;return $this->de_list();
			case 'd':	++$this->offset;return $this->de_dict();
			default :	if(ctype_digit($this->get_char())) return $this->de_string();	
		}
		return '';
	}

	private function de_int(){
		$offset_e = strpos($this->source, 'e', $this->offset);
		if($offset_e === false){
			return '';
		}

		$current_off = $this->offset;
		
		if($this->get_char($current_off) == '-'){
			++$current_off;
		}
		
		if($offset_e === $current_off){
            return '';
		}

		while($current_off < $offset_e){
            if(!ctype_digit($this->get_char($current_off))){
				return '';
			}
            ++$current_off;
		}

        $value = substr($this->source, $this->offset, $offset_e - $this->offset);
        $absolute_value = (string) abs($value);
		
		if(1 < strlen($absolute_value) && '0' == $value[0]){
			return '';
		}

        $this->offset = $offset_e + 1;
        return $value + 0;
	}
	
	private function de_string(){
        if('0' === $this->get_char() && ':' != $this->get_char($this->offset + 1)){
			return '';
		}
		
        $offset_o = strpos($this->source, ':', $this->offset);
		
		if($offset_o === false){
			return '';
		}

        $content_length = (int) substr($this->source, $this->offset, $offset_o);
        if(($content_length + $offset_o + 1) > $this->length){
			return '';
		}
		
        $value = substr($this->source, $offset_o + 1, $content_length);
        $this->offset = $offset_o + $content_length + 1;
        return $value;
	}

    private function de_list(){
        $list = array();
        $terminated = false;
        $list_offset = $this->offset;
        while($this->get_char() !== false){
            if($this->get_char() == 'e'){
                $terminated = true;
                break;
            }
            $list[] = $this->do_de();
		}
		
        if(!$terminated && $this->get_char() === false){
			return '';
		}

        $this->offset++;
        return $list;
    }
	
    private function de_dict(){
        $dict = array();
        $terminated = false;
        $dict_offset = $this->offset;
        while($this->get_char() !== false){
            if($this->get_char() == 'e'){
                $terminated = true;break;
			}
			
            $key_offset = $this->offset;
            if(!ctype_digit($this->get_char())){
				return '';
			}

            $key = $this->de_string();
			
			if(isset($dict[$key])){
				return '';
			}
			
            $dict[$key] = $this->do_de();
		}
		
		if(!$terminated && $this->get_char() === false){
			return '';
		}
		
        $this->offset++;
        return $dict;
    }

	private function get_char($offset = null){
		
		if(null != $offset){
			$_offset = $offset; 
		}else{
			$_offset = $this->offset;
		}

		return $this->source[$_offset];
	}

}



/*
if(isset($_GET['test']) && $_GET['test'] == '123'){
	opcache_reset();
	$obj = new bencode();
	$data = '{"t":"aa", "y":"q", "q":"ping", "a":{"id":"abcdefghij0123456789"}}';
	$data = json_decode($data, true);
	var_dump($data);
	echo $l = $obj->en($data);	
	$list = $obj->de($l);
	var_dump($list);
}
*/
?>

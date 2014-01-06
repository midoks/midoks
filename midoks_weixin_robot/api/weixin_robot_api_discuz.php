<?php
/**
 * 关于DISCUZ帖子信息类
 * Author: Midoks
 * Author URI: http://midoks.cachecha.com/
 */
class weixin_robot_api_discuz{
	public $obj;

	public $opt_big_show = array();
	public $opt_small_show = array();
	//架构函数
	public function __construct($obj){
		$this->obj = $obj;
		//最优图片选择是否开启
		if($this->obj->options['opt_pic_show']){
			$this->opt_pic_sign = true;
			$this->str_to_arr();//数组

		}else{
			$this->opt_pic_sign = false;
		}
	}

	public function query($sql){
		return weixin_robot_query($sql);
	}

	public function query_object($sql){
		return weixin_robot_query_object($sql);
	}

	public function str_to_arr(){
		//小图
		$small = $this->obj->options['opt_small_show'];
		if(!empty($small)){
			$this->opt_small_show = false;
		}
		$s_arr = explode("\r\n", $small);
		$tmp = array();
		foreach($s_arr as $k=>$v){
			$tmp[] = trim($v);
		}
		$this->opt_small_show = $tmp;
		//大图
		$big = $this->obj->options['opt_big_show'];
		if(!empty($big)){
			$this->opt_big_show = false;
		}
		$s_arr = explode("\r\n", $big);
		$tmp = array();
		foreach($s_arr as $k=>$v){
			$tmp[] = trim($v);
		}
		$this->opt_big_show = $tmp;

		//var_dump($this->opt_big_show, $this->opt_small_show);
	}

	//对中文名的图片路径进行urlencode编码
	public function path_url_encode($thumb){
		$pos = strrpos($thumb,'/');
		return substr($thumb, 0,$pos+1).urlencode(substr($thumb, $pos+1));
	}

	public function get_opt_pic($c, $type){
		//图片格式
		//$picType = array('jpg','gif','png','bmp');
		//foreach($picType as $v){
			//$u = '/http:\/\/(.*)\.'.$v.'/iUs';
		$u2 = '/(<img[^>]+src\s*=\s*\"?([^>\"\s]+)\"?[^>]*>)/im';
		//echo $u2;
		$p_sign = preg_match($u2 ,$c, $match);
		if($p_sign){
			//var_dump($match);
			return $this->path_url_encode($match[2]);
		}
		//}

		//上面执行过,选择默认自定义的图片
		if('small' == $type){
			$num = count($this->opt_small_show);
			$t = $num - 1;
			$mt = mt_rand(0, $t);
			if($num){
				return $this->opt_small_show[$mt];
			}
			//$tmp = $this->obj->options['opt_small_show'];
		}else if('big' == $type){
			$num = count($this->opt_big_show);
			$t = $num - 1;
			$mt = mt_rand(0, $t);
			if($num){
				return $this->opt_big_show[$mt];
			}
			//$tmp = $this->obj->options['opt_big_show'];
		}
		//midoks 默认
		return false;
	}


	/**
	 *	获取最优图片地址
	 */
	public function get_opt_pic_small($content = ''){
		if($this->opt_pic_sign){
			$pic = $this->get_opt_pic($content, 'small');
			if(!empty($pic)){
				return $pic;
			}
		}
		return $this->obj->smallPic();
	}

	/**
	 *	获取最优图片地址
	 */
	public function get_opt_pic_big($content = ''){
		if($this->opt_pic_sign){
			$pic = $this->get_opt_pic($content, 'big');
			if(!empty($pic)){
				return $pic;
			}
		}
		return $this->obj->bigPic();
	}

	//对每个第一条消息,进行处理...
	public function head_one_line($c){
		//var_dump($c);exit;
		$c = html_entity_decode($c, ENT_NOQUOTES, 'utf-8');
		$c = strip_tags($c);
		$c = mb_substr($c, 0, 50, 'utf-8').'...';
		return $c;
	}

	public function today(){
		return $this->new_art(1);
	}

	public function new5(){
		return $this->new_art(5);
	}

	public function new_art($int){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1' order by t.tid desc limit ".$int;
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 最热5篇文章信息
	 */
	public function hot5(){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select t.tid, t.subject, p.message, count(p.tid) as c from ".
			" {$pre}forum_post p left join {$pre}forum_thread t on  t.tid=p.tid group by p.tid order by c desc";
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 随机5篇文章信息
	 */
	public function rand5(){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1' order by RAND() desc limit 5";
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 指定文章回复
	 */
	public function Qid($id){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1' and t.tid='{$id}' limit 1";
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}


	//@param array $id 
	public function Qids($id){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1' and t.tid='{$id}' limit 1";
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	//获取分类列表
	public function get_category_list(){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select fid,name from ".
			"{$pre}forum_forum where status='1' limit 9";
		
		$data = $this->query($sql);
		//
		$info = array();
		$a['title'] = '欢迎光临,点击喜欢的板块';
		$a['desc'] =  '介绍';
		$a['link'] = DISCUZ_ROOT_NA;
		$info[] = $a;
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['name'];
				$a['desc'] = $this->head_one_line($v['name']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=forumdisplay&fid='.$v['fid'];
			}else{
				$a['title'] = $v['name'];
				$a['desc'] = $v['name'];
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=forumdisplay&fid='.$v['fid'];
			}
			$info[] = $a;
		}
		if(empty($info)){
			return false;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	public function total_tz(){
		global $_G;
		$pre = $_G['config']['db'][1]['tablepre'];
		//var_dump($pre);
		$sql = "select count(tid) as c from {$pre}forum_thread";
		$data = $this->query($sql);
		return $data[0]['c'];
	}

	//@func 总帖子数
	public function total(){
		$total = $this->total_tz();
		$page = ceil($total/5);
		$text = '共有:'.$total.'篇帖子'."\n";
		$text .= $page.'页(5帖一页)';
		return $this->obj->toMsgText($text);
	}

	//@func 页面浏览
	public function pageView($kw){
		//先判断是否是关键字查询
		if($kq = $this->keyQuery($kw)){
			return $kq;
		}

		$word_prefix = substr($kw, 0, 1);
		if($word_prefix!='p'){
			return false;
		}
		//var_dump($word_prefix);
		$word_suffix = substr($kw, 1);
		//var_dump(is_numeric($word_suffix));
		if(!is_numeric($word_suffix)){
			//var_dump($word_prefix);
			return false;
		}
		//var_dump($word_prefix);

		$total = $this->total_tz();
		$pageTotal = ceil($total/5);
		//var_dump($word_suffix,$pageTotal);
		if($word_suffix > $pageTotal){
			return $this->obj->toMsgText("你输入页数太大!!!");
		}

		global $_G;
		$pagestart = ($word_suffix-1)*5;
		$pre = $_G['config']['db'][1]['tablepre'];
		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1' order by t.tid desc limit {$pagestart}, 5";
		
		$data = $this->query($sql);
		//
		$info = array();
		$i = 0;
		foreach($data as $k=>$v){
			++$i;
			if($i==1){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}	
		return $this->obj->toMsgTextPic($info);//图文
	}

	//关键字查询
	public function keyQuery($kw){
		//var_dump($kw);
		//$kw = $this->convert($kw);
		//var_dump($kw);
		$word_prefix = substr($kw, 0, 1);
		//var_dump($word_prefix);
		if($word_prefix != '?'){
			return false;
		}
		$word_suffix = substr($kw, 1);
		$keyWord = explode('!', $word_suffix);
		if(empty($keyWord[1])){
			$keyWord[1] = '1';
		}
		//var_dump($keyWord);
		if(count($keyWord)==1){//关键字前5篇文章
			//return $this->keyWordSoso($keyWord);
			return $this->keyWordSoso($keyWord[0]);
		}
		//询问文章数据
		if($keyWord[1]=='?'){
			return $this->keyWordSoso($keyWord[0], '?');
		}
		//翻页功能
		if(is_numeric($keyWord[1])){
			return $this->keyWordSoso($keyWord[0], $keyWord[1]);
		}
	}

	/**
	 *	@func 关键字搜索(新 5篇)
	 *	@param $key 关键字
	 *	@ret string xml
	 */
	public function keyWordSoso($key, $sign=''){
		//判断$sign
		$res = $this->keySoso($key, $sign);
	
		if($sign == '?'){
			$num = count($res);
			$p = ceil($num/5);
			return $this->obj->toMsgText("~{$key}~关键字,共{$num}篇有{$p}页!!!");
		}
		if(!$res){
			if(empty($sign)){
				return $this->obj->toMsgText("未有~{$key}~关键字");
			}
			return $this->obj->toMsgText("~{$key}~关键字,没有第{$sign}页!!!");
		}
		//var_dump($res);exit;
		$info = array();
		foreach($res as $k=>$v){
			if(0==$k){
				$a['title'] = $v['subject'];
				$a['desc'] = $this->head_one_line($v['message']);
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}else{
				$a['title'] = $v['subject'];
				$a['desc'] = $v['message'];
				$a['pic'] = $this->get_opt_pic_big($v['message']);
				$a['link'] = DISCUZ_ROOT_NA.'forum.php?mod=viewthread&tid='.$v['tid'];
			}
			$info[] = $a;
		}
		//var_dump($info);exit;
		if(0==count($info)){
			return $this->obj->toMsgText("~{$key}~关键字,没有数据!!!");
		}
		return $this->obj->toMsgTextPic($info);
	}

	/**
	 *	@func 关键查询
	 *	@param string $k 关键字
	 *	@ret array
	 */
	public function keySoso($k, $sign){
		global $_G;
		$pagestart = ($word_suffix-1)*5;
		$pre = $_G['config']['db'][1]['tablepre'];
		
		$limit = '';
		//limit
		//判断$sign
		if(empty($sign)){
			$limit = 'limit 0, 5';
		}else if($sign=='?'){
			$limit = '';
		}else{
			$p = 5*($sign-1);
			$limit = "limit {$p},5";
		}

		$sql = "select t.tid, t.subject, p.message from ".
			"{$pre}forum_thread t left join {$pre}forum_post p on t.tid=p.tid where p.first='1'".
			////关键字处
			" and (t.subject like'%{$k}%' or p.message like '{$k}')".
			" order by t.tid desc ";
		if($sign=='?'){
			$sql_num = $sql.$limit;
			$res = $this->query($sql_num);
		}else{
			$num = $this->query($sql);
			//var_dump($num);
			$p = ceil(count($num)/5);
			//var_dump($p);
			if($sign>$p){
				return false;
			}
			$sql_num = $sql.$limit;
			$res = $this->query($sql_num);
		}
		return $res;
	}	
}
?>

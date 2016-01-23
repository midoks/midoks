<?php
/**
 * 关于DEDE文章信息类
 * Author: Midoks
 * Author URI: http://midoks.cachecha.com/
 */
class weixin_robot_api_dede{
	public $obj = null;

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
			$url = get_dede_sys_var('cfg_basehost');
			return $url.$this->path_url_encode($match[2]);
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

	//取得关联数据
	public function query($sql){
		global $dsql;
		$sql = str_replace('#@__', $dsql->dbPrefix, $sql);
		//echo $sql,"\r\n";
		$res = mysql_query($sql, $dsql->linkID);
		if($res){
			$ret = array();
			while($v = mysql_fetch_assoc($res)){
				$ret[] = $v;
			}
			return $ret;
		}
		return false;
	}

	public function today(){
		return $this->new_art(1);
	}

	public function new5(){
		return $this->new_art(5);
	}

	public function new_art($int){
		$sql = "select * from `#@__archives`  order by `id` desc limit ".$int;
		$new_data = $this->dede_query($sql, 'm');

		$info = array();
		$i = 0;
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($new_data as $k=>$row){
			//var_dump($row);
			if(!isset($row->source)){continue;}
			++$i;
	
			$title = $row->title;
			$content = $this->dede_content_get($row->id);
			$desc = $this->head_one_line($row->description);

			if('true' == $static){//静态模式
				
				$sql_type = "select * from `#@__arctype` where `id`='{$row->typeid}'";
				$data = $this->query($sql_type);
				$data = $data[0];

				$url = str_replace('{cmspath}/', $posts_page, $data['sitepath']);

				$get = date('Y/md', $row->pubdate);
				$id = $row->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$row->id;
			}

			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc; 
				$a['pic'] = $this->get_opt_pic_small($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 最热5篇文章信息
	 */
	public function hot5(){
		$sql = "select * from `#@__archives` order by `click` limit 5";
		$new_data = $this->dede_query($sql, 'm');
		$info = array();
		$i = 0;
		
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($new_data as $k=>$row){
			//var_dump($row);
			if(!isset($row->source)){continue;}
			++$i;
	
			$title = $row->title;
			$content = $this->dede_content_get($row->id);
			$desc = $this->head_one_line($row->description);

			if('true' == $static){//静态模式
				
				$sql_type = "select * from `#@__arctype` where `id`='{$row->typeid}'";
				$data = $this->query($sql_type);
				$data = $data[0];

				$url = str_replace('{cmspath}/', $posts_page, $data['sitepath']);

				$get = date('Y/md', $row->pubdate);
				$id = $row->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$row->id;
			}

			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc; 
				$a['pic'] = $this->get_opt_pic_small($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 随机5篇文章信息
	 */
	public function rand5(){
		$sql = "select * from `#@__archives` order by rand() limit 5";
		$new_data = $this->dede_query($sql, 'm');

		$info = array();
		$i = 0;
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($new_data as $k=>$row){
			//if(!isset($row->source)){continue;}
			$get = date('Y/md', $row->pubdate);
			$id = $row->id.'.html';

			$title = $row->title;
			$content = $this->dede_content_get($row->id);
			$desc = $this->head_one_line($row->description);

			if('true' == $static){//静态模式	
				$sql_type = "select * from `#@__arctype` where `id`='{$row->typeid}'";
				$data = $this->query($sql_type);
				$data = $data[0];

				$url = str_replace('{cmspath}/', $posts_page, $data['sitepath']);

				$get = date('Y/md', $row->pubdate);
				$id = $row->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$row->id;
			}

			++$i;
			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}

	/**
	 * @func 指定文章回复
	 */
	public function Qid($id){
		$sql = "select * from `#@__archives` where `id`='{$id}'";
		$new_data = $this->dede_query($sql, 'm');

		$info = array();
		$i = 0;
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($new_data as $k=>$row){
			if(!isset($row->source)){continue;}
			$get = date('Y/md', $row->pubdate);
			$id = $row->id.'.html';

			$title = $row->title;
			$content = $this->dede_content_get($row->id);
			$desc = $this->head_one_line($row->description);

			if('true' == $static){//静态模式	
				$sql_type = "select * from `#@__arctype` where `id`='{$row->typeid}'";
				$data = $this->query($sql_type);
				$data = $data[0];

				$url = str_replace('{cmspath}/', $posts_page, $data['sitepath']);

				$get = date('Y/md', $row->pubdate);
				$id = $row->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$row->id;
			}

			++$i;
			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		if(empty($info)){
			return false;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}


	//@param array $id 
	public function Qids($id){
		$sql = "select * from `#@__archives` where `id`='{$id}'";
		$new_data = $this->dede_query($sql, 'm');
		
		$info = array();
		$i = 0;
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($new_data as $k=>$row){
			if(!isset($row->source)){continue;}
			$get = date('Y/md', $row->pubdate);
			$id = $row->id.'.html';

			$title = $row->title;
			$content = $this->dede_content_get($row->id);
			$desc = $this->head_one_line($row->description);

			if('true' == $static){//静态模式	
				$sql_type = "select * from `#@__arctype` where `id`='{$row->typeid}'";
				$data = $this->query($sql_type);
				$data = $data[0];

				$url = str_replace('{cmspath}/', $posts_page, $data['sitepath']);

				$get = date('Y/md', $row->pubdate);
				$id = $row->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$row->id;
			}

			++$i;
			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		if(empty($info)){
			return false;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}

	//获取分类列表
	public function get_category_list(){
		$sql = "select * from `#@__arctype` where `topid`='0' limit 10";
		$new_data = $this->dede_query($sql, 'm');
		
		$info = array();
		$i = 0;
		$a['title'] = '欢迎光临,点击喜欢的栏目';
		$a['desc'] =  '介绍';
		//$a['pic'] = $this->get_opt_pic_big(get_the_content());
		$posts_page = DEDE_ROOT_NA;
		$a['link'] = $posts_page;
		$info[] = $a;

		foreach($new_data as $k=>$v){
			if(empty($v)){continue;}
			++$i;
			$title = $v->typename;
			if($i==1){
				$a['title'] = $title;
				$a['desc'] =  $title;
				//$a['pic'] = $this->get_opt_pic_big(get_the_content());
				//$a['link'] = str_replace('{cmspath}/', $posts_page, $v->typedir);//静态页
				$a['link'] = $posts_page.'plus/list.php?tid='.$v->id;//动态页
			}else{
				$a['title'] = $title;
				$a['desc'] =  $title;
				$a['link'] = $posts_page.'plus/list.php?tid='.$v->id;
			}
			$info[] = $a;
		}
		if(empty($info)){
			return false;
		}
		return $this->obj->toMsgTextPic($info);//图文
	}


	public function art_total(){
		$sql = "select count(id) as count from `#@__archives`";
		$new_data = $this->dede_query($sql);
		$data = $new_data->count;
		return $data;
	}

	//@func 总文章数
	public function total(){
		$total = $this->art_total();
		$page = ceil($total/5);
		$text = '共有:'.$total.'篇文章'."\n";
		$text .= $page.'页(5篇一页)';
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

	
		$sql = "select count(id) as count from `#@__archives`";
		$data = $this->dede_query($sql);
	
		$total = $data->count;
		$pageTotal = ceil($total/5);
		//var_dump($word_suffix,$pageTotal);
		if($word_suffix > $pageTotal){
			return $this->obj->toMsgText("你输入页数太大!!!");
		}

		//echo 'ci';exit;
		if($word_suffix>0){
			$word_suffix = ($word_suffix-1)*5;
		}
		$sql = "select * from `#@__archives` limit {$word_suffix}, 5";
		$data = $this->dede_query($sql,'m');
		
		//var_dump($data);
		$info = array();
		$i = 0;
		
		$posts_page = DEDE_ROOT_NA;
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($data as $k=>$v){
			++$i;

			$title = $v->title;
			$typeid = $v->typeid;
			$content = $v->description;
			$desc = $this->head_one_line($content);
			$content = $this->dede_content_get($v->id);

			if('true' == $static){//静态模式
				$sql_type = "select * from `#@__arctype` where `id`='{$typeid}'";
				$data_type = $this->query($sql_type);
				$data_type = $data_type[0];
				$url = str_replace('{cmspath}/', $posts_page, $data_type['sitepath']);

				$get = date('Y/md', $v->pubdate);
				$id = $v->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$v->id;
			}

			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_small($content);
				$a['link'] = $link;
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
		//var_dump($res);
		$info = array();
		$posts_page = DEDE_ROOT_NA;
		if(!$res[0]){
			return $this->obj->toMsgText("~{$key}~关键字,没有数据!!!");
		}
		//当前DEDE运行的模式
		$static = $this->obj->options['weixin_robot_dede_static'];
		foreach($res as $k=>$v){
			++$i;

			$title = $v->title;
			$typeid = $v->typeid;
			$content = $v->description;
			$desc = $this->head_one_line($content);
			$content = $this->dede_content_get($v->id);

			if('true' == $static){//静态模式
				$sql_type = "select * from `#@__arctype` where `id`='{$typeid}'";
				$data_type = $this->dede_query($sql_type);
				$url = str_replace('{cmspath}/', $posts_page, $data_type->sitepath);

				$get = date('Y/md', $v->pubdate);
				$id = $v->id.'.html';
				$link = $url.'/'.$get.'/'.$id;
			}else{//动态模式
				$link = $posts_page.'plus/view.php?aid='.$v->id;
			}

			if($i==1){
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_big($content);
				$a['link'] = $link;
			}else{
				$a['title'] = $title;
				$a['desc'] = $desc;
				$a['pic'] = $this->get_opt_pic_small($content);
				$a['link'] = $link;
			}
			$info[] = $a;
		}
		if(count($info)==0){
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
		global $dsql;
		$limit = '';
		//判断$sign
		if(empty($sign)){
			$limit = 'limit 0, 5';
		}else if($sign=='?'){
			$limit = '';
		}else{
			$p = 5*($sign-1);
			$limit = "limit {$p},5";
		}
	
		$sql = "SELECT * from `#@__archives` ".
			"where ".
			//关键字处
			"((keywords like '%{$k}%') ".
			"or (description like '%{$k}%') or (title like '%{$k}%'))".
			"order by `id` desc ";

		if( '?'== $sign){
			$sql_num = $sql.$limit;
			$res = $this->dede_query($sql_num);
		}else{
			$num = $this->dede_query($sql, 'm');
			//var_dump($num);
			$p = ceil(count($num)/5);
			if($sign>$p){
				return false;
			}
			$sql_num = $sql.$limit;
			$res = $this->dede_query($sql_num, 'm');
		}
		return $res;
	}



	public function dede_query($sql, $isOne = 'one'){
		global $dsql;
		$dsql->SetQuery($sql);
		$dsql->Execute();
		$data = $dsql->GetObject();

		if('one' == $isOne){
			return $data;
		}else{
			$ret = array();
			$ret[] = $data;
			while($data2 = $dsql->GetObject()){
				$ret[] = $data2;
			}
			return $ret;
		}
		//var_dump($data);
		return $data;
	}

	/**
	 * @func 根据文章id获取内容
	 */
	public function dede_content_get($id){
		$sql = "select `body` from `#@__addonarticle` where `aid`='{$id}'";
		$data = $this->dede_query($sql);
		$content = $data->body;
		return $content;
	}
		
}
?>

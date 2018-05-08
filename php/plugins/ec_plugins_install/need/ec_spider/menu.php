<?php
/**
 * @func 微信机器人后台设置
 */


if(isset($_GET['act'])){
//start

if( 'readme' == $_GET['act']){

	$smarty->display('ec_midoks_spider_readme.htm');
}elseif('list' == $_GET['act']){
	m_ec_spider_other();
	$smarty->display('ec_midoks_spider_list.htm');
}



//end
}


/**
 *	@func 控制其他的请求
 */
function m_ec_spider_other(){
	global $smarty;
	

	//删除数据
	if(isset($_GET['id']) && isset($_GET['method']) && 'del' == $_GET['method']){
		$id = $_GET['id'];
		m_ec_spide_del_id($id);
		header('Location: '.$_SERVER['HTTP_REFERER']);
	}

	//清空数据
	if(isset($_GET['method']) && 'clear'==$_GET['method']){
		m_ec_spide_clear();
	}

	//数据分页
	
	$num = 20;
	$c = m_ec_spider_list_c();
	$page_num = ceil($c/$num);
	if(isset($_GET['p'])){
	 	$p = (int)$_GET['p'];
		if($p<1){
			$p=1;
		}elseif($p>$page_num){
			$p = $page_num;
		}
		
	}

	$page_str = m_ec_page($c, $p, $num, 7, $pn = 'p');
	$smarty->assign('page', $page_str);

	$list = m_ec_spider_list($p, $num);
	if($list){
		$smarty->assign('list', $list);
	}else{
		$smarty->assign('error', '没有任何数据!!!');
	}

}

/**
 *	@func 获取蜘蛛记录
 *	@ret array
 */
function m_ec_spider_list($page=1, $num=20){
	global $ecs, $db;
	$table = $ecs->table('midoks_spider');
	
	if($page<1){$page=1;}
	$page_start = ($page-1)*$num;
	$sql = "select * from {$table} order by id desc limit {$page_start}, {$num}";
	$list = $db->getAll($sql);
	if(!empty($list)){
		foreach($list as $k=>$v){
			$list[$k]['time'] = date('Y-m-d H:i:s', $v['time']);
		}
		return $list;
	}
	return false;
}


function m_ec_spider_list_c(){
	global $ecs, $db;
	$table = $ecs->table('midoks_spider');
	$sql = "select count(id) from {$table}";
	$c = $db->getOne($sql);
	if($c){
		return $c;
	}
	return false;
}

function m_ec_spide_del_id($id){
	global $ecs, $db;
	$table = $ecs->table('midoks_spider');
	$sql = "delete from {$table} where id='{$id}'";
	return $db->query($sql);
}

function m_ec_spide_clear(){
	global $ecs, $db;
	$table = $ecs->table('midoks_spider');
	$sql = "truncate table {$table}";
	$db->query($sql);
}


function m_ec_page($total, $position, $page=5, $show=7, $pn = 'nav'){
		//当前页
		$url = $_SERVER['REQUEST_URI'];
		$r_url = str_replace(strstr($url, '&'), '', $url);
		$thisPageUrl = 'http://'.$_SERVER['HTTP_HOST'].$r_url.'&file='.$_GET['file'].'&act='.$_GET['act'].'&'.$pn.'=';
		$ret .= '';	
		
		$prev = $position-1;//前页
		$next = $position+1;//下页
		//$showitems = 3;//显示多少li
		$big = ceil($show/2);
		$small = floor($show/2);//$show最好为奇数 
		$total_page = ceil($total/$page);//总页数
		//if($prev < 1){$prev = 1;}
		if($next > $total_page){$next = $total;}
		if($position > $total_page){$position = $total_page;}
		if(0 != $total_page){
			//echo "<div id='page'><div class='center'><ul>";
			/////////////////////////////////////////////
			$ret .= ("<span><a href='".$thisPageUrl."1#' class='fixed'>首页</a></span>");
			$ret .= ("<span style='margin-left:5px;'><a href='".$thisPageUrl.$prev."#'><<</a></span>");
			$j=0;
			for($i=1;$i<=$total_page;$i++){
				$url = $thisPageUrl.$i;
				if($position==$i)
					$strli = "<span style='margin-left:5px;'><a href='".$url."#' class='current' >".$i.'</a></span>';
				else
					$strli = "<span style='margin-left:5px;'><a href='".$url."#' class='inactive' >".$i.'</a></span>';
				if($total_page<=$show){$ret .= $strli;}
				if(($position+$small)>=$total_page){
					if(($j<$show) && ($total_page>$show) && ($i>=($total_page-(2*$small)))){$ret .= ($strli);++$j;}
				}else{if(($j<$show) && ($total_page>$show) && ($i>=($position-$small))){$ret .= ($strli);++$j;}}
			}
			$ret .= ("<span style='margin-left:5px;'><a href='".$thisPageUrl.$next."#'>>></a></span>");
			$ret .= ("<span style='margin-left:5px;'><a href='".$thisPageUrl.$total_page."#' class='fixed'>尾页</a></span>");

			$ret .= ("<span style='margin-left:30px;'>共{$total}条数据|</span>");
			$ret .= ("<span>当前第{$position}页</span>");
			//////////////////////////////////////////////
			//echo '</ul></div></div>';
		}
		//$ret .= ('');
		return $ret;
}
?>

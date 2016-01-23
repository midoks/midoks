<?php
include('config.php');

class weixin_robot_stat{

	public $db = null;

	//构造函数
	public function __construct(){
		include(WEIXIN_ROOT_API.'weixin_robot_api_discuz_db.php');
		$this->db = new weixin_robot_api_discuz_db();
	}

	public function run(){

		//var_dump($_SERVER);

		$data = $this->db->weixin_get_count();
		//当前页
		$paged = isset($_GET['paged']) ? $_GET['paged'] : 1;
		//每页显示多少数据
		$pageNum = 20;
		$db = $this->db;
		$c = $this->db->weixin_get_count();
		$pagePos = ceil($c/$pageNum);
		if($paged > $c){
			$paged = $c;
		}
		if($paged < 1){
			$paged = 1;
		}
		$trTpl = "<tr align='center' bgcolor='#FBFCE2' height='24'>
			<td>%s</td>
			<td >%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td></tr>";
		$tableHeadTpl = sprintf($trTpl, '序号ID', '开发者ID', '用户ID',
			'消息类型', '消息内容', '消息时间', '回复');




		$tableTrTpl = "<tr align=\"center\" bgcolor=\"#FFFFFF\" height=\"24\" onmousemove=\"javascript:this.bgColor='#FCFDEE';\" onmouseout=\"javascript:this.bgColor='#FFFFFF';\">
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td></tr>";
		$tableBodyTpl = '';
		$data = $db->weixin_get_data($paged);
	
		foreach($data as $k=>$v){
				//var_dump($v);
			$tableBodyTpl .= sprintf($tableTrTpl,   $v['id'], $v['to'], $v['from'],
				$this->type_replace($v['msgtype']), $v['content'], $v['createtime'], $v['response']);
		}



		echo '<table width="98%" border="0" align="center" cellpadding="3" cellspacing="1" bgcolor="#D6D6D6">';
		echo '<thead>';
		echo '<tr><td height="28" colspan="7" style="padding-left:10px;"><b>微信通信记录</b></td></tr>';
		echo($tableHeadTpl);
		echo '</thead>';
		
		echo '<tbody>';
		echo($tableBodyTpl);
		echo '</tbody>';

		echo '<tfoot>';
		//分页显示
		echo '<tr align="center" bgcolor="#F9FCEF" height="24"><td colspan="7">';
		//echo($this->weixin_info_page($c, $paged, $pageNum));
		echo '</td></tr>';
		echo '</tfoot>';
		echo '</table>';
	}

	public function type_replace($type){
		switch($type){
			//文本消息	
			case 'text':return '文本';break;
			//图片消息
			case 'image':return '图片';break;
			//语音消息
			case 'voice':return '语音';break;
			//视频消息
			case 'video':return '视频';break;
			//事件消息
			case 'event':return '事件';break;
			//地理位置
			case 'location': return '地理';break;
			case 'link':return '连接';break;
			//默认消息
			default:return '文本';break;
		}
		return '你傻了吧';
	}



	/**
	 * @func  分页功能 path版
	 * @param $total 	共多少数据
	 * @param $position 在第几页
	 * @param $page 	每页的数量
	 * @param $show  	显示多少li
	 */
	public function weixin_info_page($total, $position, $page=5, $show=7){
		$prev = $position-1;//前页
		$next = $position+1;//下页
		//$showitems = 3;//显示多少li
		$big = ceil($show/2);
		$small = floor($show/2);//$show最好为奇数 
		$total_page = ceil($total/$page);//总页数
		//if($prev < 1){$prev = 1;}
		if($next > $total_page){$next = $total_page;}
		if($position > $total_page){$position = $total_page;}

		$_GET['paged'] = null;
		$current_page = $this->get_pages();
		if(0 != $total_page){
			echo "<div>";
			echo("<span>总共{$total}页/当前第{$position}页<span>");
			/////////////////////////////////////////////
			echo("<span style='margin-left:30px'><a href='".$current_page.'&paged=1'."#' class='fixed'>首页</a></span>");
			echo("<span style='margin-left:30px'><a class='p_prev' href='".$current_page.'&paged='.$prev."#'><<</a></span>");
			$j=0;
			for($i=1;$i<=$total_page;$i++){
				$url = $current_page.'&paged='.$i;
				if($position==$i)
					$strli = "<span style='margin-left:30px'><a href='".$url."#' class='current' >".$i.'</a></span>';
				else
					$strli =  "<span style='margin-left:30px'><a href='".$url."#' class='inactive' >".$i.'</a></span>';
				if($total_page<=$show){echo $strli;}
				if(($position+$small)>=$total_page){
					//也是对的,下面为简化版
					//if(($j<$show) && ($total_page>$show) && ($i>=($position-($small+($position+$small-$total_page))))){echo($strli);++$j;}
					if(($j<$show) && ($total_page>$show) && ($i>=($total_page-(2*$small)))){echo($strli);++$j;}
				}else{if(($j<$show) && ($total_page>$show) && ($i>=($position-$small))){echo($strli);++$j;}}
			}
			echo("<span style='margin-left:30px'><a class='p_next' href='".$current_page.'&paged='.$next."#'>>></a></span>");
			echo("<span style='margin-left:30px'><a href='".$current_page.'&paged='.$total_page."#'>尾页</a></span>");
			//////////////////////////////////////////////
			echo '</div>';
		}
	}


	//去除?号之后的数据
	public function get_pages(){
		//$hurl = 'http://'.$_SERVER['HTTP_HOST'];
		$durl = '';

		$url = $hurl.$_SERVER['REQUEST_URI'];
		$pos = strpos($url, '&');
		if(false===$pos){
			return $url;
		}
		$url = substr($url, 0, $pos);
		return $url;
	}

}
$obj = new weixin_robot_stat();
$obj->run();
?>

<?php
include('config.php');

/**
 *	@func 自定义菜单设置
 *	@author midoks
 *	@blog midoks.cachecha.com
 */
class weixin_robot_menu_setting{

	public $db = null;
	public $options;

	public function __construct(){
		include_once(WEIXIN_ROOT_API.'weixin_robot_api_discuz_db.php');
		$this->db = new weixin_robot_api_discuz_db();

		$this->options = get_option('weixin_robot_options');
	}

	public function run(){
		
		echo '<form action="" method="POST">';
		
		echo '<table width="98%" border="0" align="center" cellpadding="3" cellspacing="1" bgcolor="#D6D6D6">';
		echo '<thead>';

		//关键字回复
		$this->_keyPost();
		$this->keyReply();

		$opts = $this->options;
		if((!empty($opts['ai'])) || (!empty($opts['as']))){
			$this->_menuPost();
			//菜单设置
			$this->menuSetting();
		}else{
			
			echo '<tr><td colspan="5" align="center" bgcolor="#FFFFFF">';
			echo '<p style="color:red">没有设置appID和appsecret,所有没有自定义菜单设置</p>';
			echo '</td></tr>';
		}
		//结束
		echo '<tbody>';
		echo '</table></body>';
		echo '</form>';
	}

	public function notices($str){
		echo '<tr><td colspan="5" align="center" bgcolor="#FFFFFF">';
		echo '<p style="color:red">'.$str.'</p>';
		echo '</td></tr>';
	}


	public function _keyPost(){
		if(isset($_POST['submit_key'])){
			switch($_POST['submit_key']){
			case '启用':
				$id = $_POST['id'];
				$data = $this->db->change_relpy_status($id, '1');
				break;
			case '禁用':
				$id = $_POST['id'];
				$data = $this->db->change_relpy_status($id, '0');
				break;
			case '删除':
				$id = $_POST['id'];
				$data = $this->db->delete_relpy_id($id);
				break;
			case '提交数据':
				$type = $_POST['option']['check'];
				$key = $_POST['option']['key'];
				$relpy = $_POST['option']['word'];
				//$data = $this->db->insert_relpy($key, $relpy, $status=1, $time='0000-00-00 00:00:00', $type);
				if(empty($type) || empty($key) || empty($relpy)){
					$this->notices('关键字和回复信息不能为空!!!');
				}else{
					$data = $this->db->insert_relpy($key, $relpy, $status=1, $time='0000-00-00 00:00:00', $type);
					//var_dump($data);
				}

				if(!$data){
					$this->notices('关键字回复设置没有成功!!!');
				}	
				break;
			}	
		}
	}


	/**
	 *	@func 关键字回复
	 */
	public function keyReply(){
	

		$trTpl = "<tr align='center' bgcolor='#FBFCE2' height='24'>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td></tr>";
		$tableHeadTpl = sprintf($trTpl, '序号ID', '关键字', '回复内容', '类型', '操作');

		//头信息显示
		echo '<tr><td height="28" align="center" colspan="7" style="padding-left:10px;"><b>微信关键字自定义回复</b></td></tr>';
		echo($tableHeadTpl);

		$data = $this->db->weixin_get_relpy_data();
		if($data){
			foreach($data as $k=>$v){
				$trTpl = "<tr>
				<td align='center' bgcolor='#FFFFFF'>{$v['id']}</td>
				<td align='center' bgcolor='#FFFFFF'>{$v['keyword']}</td>
				<td align='center' bgcolor='#FFFFFF'>{$v['relpy']}</td>
				<td align='center' bgcolor='#FFFFFF'>{$v['type']}</td>
				<td align='center' bgcolor='#FFFFFF'>";

				$trTpl .= '<input type="hidden" name="id" value="'.$v['id'].'" />';
				$trTpl .= '<input name="submit_key" type="submit"  value="';
				if($v['status']){
					$trTpl .= '禁用';
				}else{
					$trTpl .= '启用';
				}
				$trTpl .= '" />';
				$trTpl .=" | ";
				$trTpl .= '<input name="submit_key" type="submit" value="删除" />';

				$trTpl .= "</td></tr>";
				echo '<form  method="POST">';
				echo  $trTpl;
				echo '</form>';
			}
		}else{
			echo '<tr><td align="center" bgcolor="#FFFFFF" colspan="5"><b>暂时没有设置KeyWord</b></td></tr>';
		}

		echo '</thead>';
		echo '<tbody>';
		//类型选择
		echo "<tr align='left' bgcolor='#FFFFFF'><td align='center' bgcolor='#FFFFFF'>类型选择</td><td colspan='4'>";
		echo '<select name="option[check]" id="method" />';
$select = <<<STR
			<option value="text" selected="selected">文本回复</option>
			<option value="id">图文ID回复</option>
			</select><p></p>
STR;
		echo $select;
		echo "</td></tr>";

		//关键字
		echo "<tr align='center' bgcolor='#FFFFFF' height='24'><td align='center' bgcolor='#FFFFFF'>关键字</td><td align='left' bgcolor='#FFFFFF' height='24' colspan='4'>";
		echo '<textarea name="option[key]" rows="3" id="menustring" style="width:80%"></textarea>';
		echo "</td></tr>";

		//回复信息
		echo '<tr align="center" bgcolor="#FFFFFF" height="24"><td>回复信息</td><td colspan="4"  align="left" bgcolor="#FFFFFF">';
		echo '<textarea name="option[word]" rows="3" id="menustring" style="width:80%"></textarea>';

		echo '<br/>如果选择"图文ID"选项,应该填写文章ID: 1,4,8(图文最多显示10个信息)<br/>如果选择"文本回复"选项,可以是使用0,today,n5, h5, r5, ?等内置命令!!<br/>不满足上面的话,则会返回文本信息<br/></td></tr>';


		echo '<tr><td colspan="5" align="center" bgcolor="#FFFFFF" height="24">';
		echo '<input name="submit_key" type="submit" value="提交数据">';
		echo '</td></tr>';
	}

	public function weixin_robot_menu_api($ai, $as){
		include(WEIXIN_ROOT_LIB.'weixin_robot_menu_api.php');
		$api = new weixin_robot_menu_api($ai, $as);
		return $api; 
	}

	//组装menu菜单
	public function weixin_robot_ab_menu(){
		if($data = $this->db->weixin_get_menu_p_data()){
			$string = '{"button":[';
			foreach($data as $k=>$v){
				if($data2 = $this->db->weixin_get_menu_p_data_id($v['id'])){
					$string .= '{';
					$string .= '"name":"'.$v['menu_name'].'",';
					$string .= '"sub_button":[';
					foreach($data2 as $k1=>$v2){
						$string .= '{';
						$string .= '"type":"'.$v2['menu_type'].'",';
						$string .= '"name":"'.$v2['menu_name'].'",';
						if('view' == $v2['menu_type']){
							$string .= '"url":"'.$v2['menu_callback'].'"';
						}else{
							$string .= '"key":"'.$v2['menu_key'].'"';
						}
						$string .= '},';
					}
					$string .= ']},';
				}else{
					$string .= '{';
					$string .= '"type":"'.$v['menu_type'].'",';
					$string .= '"name":"'.$v['menu_name'].'",';
					if('view' == $v['menu_type']){
						$string .= '"view":"'.$v['menu_callback'].'"';
					}else{
						$string .= '"key":"'.$v['menu_key'].'"';
					}
					$string .= '},';
				}
			}
			$string .= ']}';
			return $string;
		}
		return false;
	}


	//随机key菜单值
	public function weixin_robot_rand_menu(){
		return 'MENU_'.time();
	}

	public function _menuPost(){
		$opts = $this->options;
		$api = $this->weixin_robot_menu_api($opts['ai'], $opts['as']);
		//判断
		if(isset($_POST['submit_menu'])){
			switch($_POST['submit_menu']){
			case '提交菜单':
				$data = $_POST['weixin_robot_menu'];
				if(empty($data['name']) || empty($data['value'])){
						$this->notices('请填写号内容!!!');
					}else{
						//判断是否为1级菜单
						if('true' == $data['child'] && $data['parent'] != 'false'){//子菜单
							if($this->db->weixin_get_menu_c_count($data['parent']) < 5){
								$data = $this->db->insert_menu($data['name'], $data['type'], $this->weixin_robot_rand_menu(), $data['value'], $data['parent']);
							}else{
								$this->notices('二级菜单不能再添加了!!!');
							}
						}else{//一级菜单
							if($this->db->weixin_get_menu_p_count() < 3){
								$data = $this->db->insert_menu($data['name'], $data['type'],  $this->weixin_robot_rand_menu(), $data['value'], 0);
							}else{
								$this->notices('一级菜单不能再添加了!!!');
							}
						}
					}
				break;
			case '删除菜单':
				if($data = $api->menuDel()){
					$this->notices('删除成功!!!');
				}else{
					$this->notices('删除失败!!!');
				}
				break;
			case '同步到微信':
				$json = $this->weixin_robot_ab_menu();
				if($json){
					$data = $api->menuSet($json);
					if($data){
						$this->notices('同步成功!!!');
					}else{
						$this->notices('同步失败!!!');
					}
				}
				break;
			case '删除':
				if(isset($_POST['id']))
					if($data = $this->db->delete_menu_id($_POST['id'])){
						$this->notices('ok!!!');
					}else{
						$this->notices('fail!!!');
					}
				break;
			case '编辑':break;
			}
		}
	}

	public function menuSetting(){

		//菜单设置
		echo '<tr><td height="28" align="center" colspan="7" style="padding-left:10px;"><b>微信机器人自定义菜单设置</b></td></tr>';

		$trTpl = "<tr align='center' bgcolor='#FBFCE2' height='24'>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td>
			<td>%s</td></tr>";
		$tableHeadTpl = sprintf($trTpl, '序号ID', '菜单名', '菜单类型', 'key/url', '操作');
		echo $tableHeadTpl;



		//一级菜单
		$data = $this->db->weixin_get_menu_p_data();
		if($data){
			foreach($data as $k=>$v){
				$trTpl = "<tr><td  align='center' bgcolor='#FFFFFF'>{$v['id']}</td>
				<td align='left' bgcolor='#FFFFFF'>─{$v['menu_name']}</td>
				<td align='center' bgcolor='#FFFFFF'>{$v['menu_type']}</td>
				<td align='center' bgcolor='#FFFFFF'>{$v['menu_callback']}</td>
				<td align='center' bgcolor='#FFFFFF'>";
				$trTpl .= '<input type="hidden" name="id" value="'.$v['id'].'" />';
				$trTpl .= '<input name="submit_menu" type="submit" value="删除" />';
				$trTpl .= "</td></tr>";
				echo '<form  method="POST">';
				echo $trTpl;
				echo '</form>';
				//二级菜单
				if($data2 = $this->db->weixin_get_menu_p_data_id($v['id'])){
					foreach($data2 as $k=>$v){
						$trTpl = "<tr><td  align='center' bgcolor='#FFFFFF'>{$v['id']}</td>
						<td align='left' bgcolor='#FFFFFF'>└─{$v['menu_name']}</td>
						<td align='center' bgcolor='#FFFFFF'>{$v['menu_type']}</td>
						<td align='center' bgcolor='#FFFFFF'>{$v['menu_callback']}</td>
						<td align='center' bgcolor='#FFFFFF'>";
						$trTpl .= '<input type="hidden" name="id" value="'.$v['id'].'" />';
						$trTpl .= '<input name="submit_menu" type="submit" value="删除" />';
						$trTpl .= "</td></tr>";

						echo '<form  method="POST">';
						echo $trTpl;
						echo '</form>';
					}
				}
			}
		}else{//暂时没有菜单
			echo '<tr><td height="28" align="center" bgcolor="#FFFFFF" colspan="5" style="padding-left:10px;"><b>暂时没有设置菜单</b></td></tr>';
		}

		//菜单名称
		echo "<tr align='left' bgcolor='#FFFFFF' height='24'><td align='center'  bgcolor='#FFFFFF'>菜单名称</td><td colspan='4'>";
		echo '<input name="weixin_robot_menu[name]" type="text">';
		echo "</td></tr>";

		//事件类型选择
		echo "<tr align='left'  bgcolor='#FFFFFF' height='24'><td align='center'  bgcolor='#FFFFFF'>事件类型选择</td><td colspan='4'>";
		echo '<select name="weixin_robot_menu[type]" id="method" />';
$select = <<<STR
			<option value="click" selected="selected">点击</option>
			<option value="view" >URL</option>
			</select><p></p>
STR;
		echo $select;	
		echo "</td></tr>";

		//key/url
		echo "<tr align='left' bgcolor='#FFFFFF' height='24'><td align='center'  bgcolor='#FFFFFF'>key/url</td><td colspan='4'>";
		echo '<input name="weixin_robot_menu[value]" type="text">';
		echo '<br/>';
		echo '<p>如果选择"URL"选项,应该填写网址: http://midoks.cachecha.com/</p>';
		echo '<p>如果选择"点击"选项,可以是使用0,today,n5, h5, r5, ?等内置命令!!</p>';
		echo '<p>不满足上面的话,则会返回文本信息</p>';
		echo "</td></tr>";


		//是否为子菜单
		echo "<tr align='left' bgcolor='#FFFFFF' height='24'><td align='center'  bgcolor='#FFFFFF'>是否为子菜单</td><td colspan='4'>";
		echo '<input type="checkbox" name="weixin_robot_menu[child]"  value="true"/>';
		echo '<br />';
		echo '为子菜单时,请一定选择';
		echo "</td></tr>";

		//选择父级菜单
		echo "<tr bgcolor='#FFFFFF' height='24'><td align='center' bgcolor='#FFFFFF'>选择父级菜单</td>";
		echo '<td colspan="3">';
		echo '<select name="weixin_robot_menu[parent]" id="method" />';
		$data = $this->db->weixin_get_menu_p_data();
		if($data){
			foreach($data as $k=>$v){
				if(0==$k){
					echo "<option value='{$v['id']}' selected='selected'>{$v['menu_name']}</option>";
				}else{
					echo "<option value='{$v['id']}'>{$v['menu_name']}</option>";
				}
			}
		}else{
			echo '<option value="false" selected="selected">无顶级菜单,请先创建</option>';
		}	
		echo '</select><p></p>';
		echo '<td></tr>';

		//功能
		echo '<tr bgcolor="#FFFFFF" height="24"><td  colspan="5" align="center" bgcolor="#FFFFFF">';
		echo '<input name="submit_menu" type="submit" value="提交菜单">';
		echo '<input style="margin-left:10px" name="submit_menu" type="submit" class="button-primary" value="删除菜单" />';
		echo '<input style="margin-left:10px" name="submit_menu" type="submit" class="button-primary" value="同步到微信" />';
		echo '</td></tr>';
	}

}

$obj = new weixin_robot_menu_setting();
$obj->run();
?>

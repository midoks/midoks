<?php
class weixin_robot_setting{

	public function _post(){
		if(isset($_POST['submit'])){
			$options = $_POST['options'];

			//echo '<pre>';
			//var_dump($options);
			//echo '</pre>';

			update_option('weixin_robot_options', $options);
		}
	}

	public function run(){

		$this->_post();
		$options = get_option('weixin_robot_options');
		$dede_url = DEDE_ROOT_NA.'dede/';

		echo '<head><meta http-equiv="Content-Type" content="text/html; charset=utf-8">
			<title></title><link href="'.$dede_url.'css/base.css" rel="stylesheet" type="text/css"></head>
			<body background="'.$dede_url.'images/allbg.gif" leftmargin="8" topmargin="8">';


		echo '<form action="" method="POST">';
		echo '<table width="98%" border="0" align="center" cellpadding="3" cellspacing="1" bgcolor="#D6D6D6">';
		echo '<thead>';
		echo '<tr><td height="28" background="'.$dede_url.'images/tbg.gif" colspan="7" style="padding-left:10px;"><b>微信机器人设置</b></td></tr>';
		//echo($tableHeadTpl);
		echo '</thead>';
		
		echo '<tbody>';
		//echo($tableBodyTpl);
		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF"><b>基本设置<b/></td></tr>';


		//DEDE静态
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>DEDE静态</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="checkbox" name="options[weixin_robot_dede_static]" value="true"';
		if( $options['weixin_robot_dede_static'] == 'true' ){ echo ' checked="checked"'; }
		echo '/>当前DEDE运行的模式</td></td></tr>';

		//订阅事件提示
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>订阅事件提示</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<textarea name="options[subscribe]" rows="3" id="menustring" style="width:80%">'.$options['subscribe'].'</textarea>';
		echo '<br/> 当用户关注时,发送的消息</td></td></tr>';


		//图片最优显示
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>图片最优显示</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="checkbox" name="options[opt_pic_show]" value="true"';
		if( $options['opt_pic_show'] == 'true' ){ echo ' checked="checked"'; }
		echo '/><br />是否开启最优图片获取.<br />1.开启后会在文章中匹配第一个张图片(如果有多张图片). <br />2.如果没有找到,返回你的下面默认大小图片地址 <br />3.如过默认大小也没有设置,会返会本插件自带图片 <br /><b>note:开启图片防盗链的话,还是不要开启为好.覆盖原来的图片就很好!</b></td></td></tr>';

		//大图显示地址
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>大图显示地址</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<textarea name="options[opt_big_show]" rows="3" id="menustring" style="width:80%">'.$options['opt_big_show'].'</textarea>';
		echo '<br/>多个图片地址,回车换行来区分|官方建议大图为:360*200</td></td></tr>';

		//小图图显示地址
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>小图显示地址</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<textarea name="options[opt_small_show]" rows="3" id="menustring" style="width:80%">'.$options['opt_small_show'].'</textarea>';
		echo '<br/>多个图片地址,回车换行来区分|官方建议大图为:360*200</td></td></tr>';

		//是否开启数据记录
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>是否开启数据记录</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="checkbox" name="options[weixin_robot_record]" value="true"';
		if( $options['weixin_robot_record'] == 'true' ){ echo ' checked="checked"'; }
		echo '</td></td></tr>';

		//是否开启测试模式
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>是否开启测试模式</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="checkbox" name="options[weixin_robot_debug]" value="true"';
		if( $options['weixin_robot_debug'] == 'true' ){ echo ' checked="checked"'; }
		echo '/></td></td></tr>';

		//帮助信息
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>帮助信息</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<textarea name="options[weixin_robot_helper]" style="width:350px;height:350px;" id="menustring" style="width:80%">'.
			$options['weixin_robot_helper'].'</textarea>';
		echo '<br/>帮助信息(note:微信一行12字左右)</td></td></tr>';

		//是否启动无此匹配命令不回复
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>是否启动无此匹配命令不回复</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="checkbox" name="options[weixin_robot_helper_is]" value="true"';
		if( $options['weixin_robot_helper_is'] == 'true' ){ echo ' checked="checked"'; }
		echo '/><br>开启后,只有?回复帮助信息</td></td></tr>';

		//服务号设置
		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF"><b>服务号设置<b/></td></tr>';
		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF"><b>说明:如果你不是服务号,就不需要设置<b/></td></tr>';

		//appID
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>appID</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="text" name="options[ai]" value="';
		echo $options['ai'].'">';
		echo '<br>微信公众平台开发者ID(第三方用户唯一凭证)</td></td></tr>';

		//appsecret
		echo '<tr><td align="center" bgcolor="#FFFFFF" width="19%"><b>appsecret</b></td><td bgcolor="#FFFFFF" width="81%">';
		echo '<input type="text" name="options[as]" value="';
		echo $options['as'].'">';
		echo '<br>appsecret(第三方用户唯一凭证密钥)</td></td></tr>';


		//按钮
		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF">';
		echo '<input name="submit" type="submit" class="np" width="60" height="22" border="0" value="提交数据">';
		echo '</td></tr>';

		echo '</tbody>';
		echo '<tfoot>';
		//分页显示
		echo '<tr align="center" bgcolor="#F9FCEF" height="24"><td colspan="7">';
		//echo($this->weixin_info_page($c, $paged, $pageNum));
		echo '</td></tr>';
		echo '</tfoot>';
		echo '</table>';
		echo '</form>';
		
		echo '</body>';
	}
}
?>

<?php
class weixin_robot_count{

	public $db = null;

	public function __construct(){
		include_once(WEIXIN_ROOT_API.'weixin_robot_api_dede_db.php');
		$this->db = new weixin_robot_api_dede_db();
	}

	public function run(){
		$dede_url = DEDE_ROOT_NA.'dede/';
		echo '<head><meta http-equiv="Content-Type" content="text/html; charset=utf-8">
			<title></title><link href="'.$dede_url.'css/base.css" rel="stylesheet" type="text/css"></head>
			<body background="'.$dede_url.'images/allbg.gif" leftmargin="8" topmargin="8">';

		//头信息显示
		echo '<table width="98%" border="0" align="center" cellpadding="3" cellspacing="1" bgcolor="#D6D6D6">';
		echo '<thead>';
		echo '<tr><td height="28" background="'.$dede_url.'images/tbg.gif" colspan="7" style="padding-left:10px;"><b>微信记录统计</b></td></tr>';
		//echo($tableHeadTpl);
		echo '</thead>';

		echo '<tbody>';
		//echo($tableBodyTpl);
		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF"><b>图形展示<b/></td></tr>';


		echo '<tr><td colspan="2" align="center"  bgcolor="#FFFFFF">';
		//使用项目ichatjs | link:www.ichatjs.com
		echo '<div id="canvasDiv1"></div>';

		echo '</td></tr>';
		echo '<tbody>';

		echo '</table></body>';


		echo '<script src="'.WEIXIN_ROOT_URL.'admin/ichart.min.js'.'" language="javascript" type="text/javascript"></script>';
		//就下面为js渲染数据和图形(总统计)
		
		$db = $this->db;
		$text = $db->weixin_get_msgtype_count('text');
		$voice = $db->weixin_get_msgtype_count('voice');
		$video = $db->weixin_get_msgtype_count('video');
		$link = $db->weixin_get_msgtype_count('link');
		$event = $db->weixin_get_msgtype_count('event');
		$image = $db->weixin_get_msgtype_count('image');
		$location = $db->weixin_get_msgtype_count('location');
		echo "<script>
		$(function(){
			var data = [
			    {name : '文本信息',value : {$text}, color:'#fedd74'},
				{name : '音频消息',value : {$voice}, color:'#82d8ef'},
			    {name : '视频消息',value : {$video}, color:'#f76864'},
				{name : '链接消息',value : {$link}, color:'#80bd91'},
				{name : '事件消息',value : {$event}, color:'#80ee91'},
				{name : '地理消息',value : {$location}, color:'#70bc91'},
			    {name : '图片消息',value : {$image}, color:'#fd9fc1'}];

	    	
			var chart = new iChart.Pie3D({
				render : 'canvasDiv1',
				data: data,
				title : {
					text : '微信通信记录统计',
					color : '#3e576f'
				},
				footnote : {
					text : '感谢www.ichartjs.com提供',
					color : '#486c8f',
					fontsize : 11,
					padding : '0 38'
				},
				bound_event:null,
				sub_option : {
					label : {
						background_color:null,
						sign:false,//设置禁用label的小图标
						padding:'0 4',
						border:{
							enable:false,
							color:'#be5985'
						},
						fontsize:11,
						fontweight:600,
						color : '#be5985'
					},
					border : {
						width : 2,
						color : '#ffffff'
					}
				},
				shadow : true,
				shadow_blur : 6,
				shadow_color : '#aaaaaa',
				shadow_offsetx : 0,
				shadow_offsety : 0,
				background_color:'#fefefe',
				yHeight:20,//饼图厚度
				offsetx:60,//设置向x轴负方向偏移位置60px
				offset_angle:0,//逆时针偏移120度
				mutex : true,//只允许一个扇形弹出
				showpercent:true,
				decimalsnum:2,
				width : 800,
				height : 400,
				radius:150
			});
			chart.plugin(new iChart.Custom({
					drawFn:function(){
						//计算位置
						var y = chart.get('originy'),
							w = chart.get('width');
						chart.target.textAlign('start')
						.textBaseline('middle')
						.textFont('600 16px Verdana')
						.fillText('微信统计各类型比',60,y-40,false,'#3e576f',false,20);
					}
			}));
			
			chart.draw();
		});		
		</script>";
	}
}
?>

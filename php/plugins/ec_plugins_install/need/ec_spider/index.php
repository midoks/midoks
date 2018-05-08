<?php
/**
 *  code:EC蜘蛛记录
 *	author:midoks
 *	website:http://midoks.cachecha.com/
 *	version:1.0
 *	desc:记录各搜索引擎抓取
 */

define('M_EC_SPIDER_ROOT', str_replace('\\', '/', dirname(__FILE__)).'/');
define('M_EC_SPIDER_P_ROOT', dirname(dirname(dirname(__FILE__))).'/templates/');

class ec_spider{
	public function run(){
		//前台记录
		if(!is_admin()){

			$user_agent = $_SERVER['HTTP_USER_AGENT'];
			$spider_list = $this->spiderList();
			$url = 'http://'.$_SERVER['HTTP_HOST'].$_SERVER['REQUEST_URI'];//蜘蛛访问的页面
			foreach($spider_list as $k=>$v){
				if((preg_match('/'.$k.'/i', $user_agent))){
					$this->insertValue( empty($v) ? 'test' : $v, $url);
					break;
				}
			}
			//$this->insertValue('test', $url);
		}
	}

	//安装插件
	public function install(){
		global $ecs, $db;
		$table = $ecs->table('midoks_spider');
		$sql = "create table if not exists {$table}(
					`id` int(10) not null auto_increment comment 'ID',
					`name` varchar(50) not null comment '蜘蛛名字',
					`time` varchar(13) not null comment '时间',
					`ip` varchar(15) not null comment 'IP地址',
					`url` varchar(255) not null comment '收录地址',
					primary key(`id`)
				)engine=MyISAM default character set utf8 comment='蜘蛛记录表' collate utf8_general_ci;";
		$res = $db->query($sql);

		//移动模版数据
		$so = M_EC_SPIDER_ROOT.'ec_midoks_spider_list.htm';
		$to = M_EC_SPIDER_P_ROOT.'ec_midoks_spider_list.htm';
		copy($so, $to);
		$so = M_EC_SPIDER_ROOT.'ec_midoks_spider_readme.htm';
		$to = M_EC_SPIDER_P_ROOT.'ec_midoks_spider_readme.htm';
		copy($so, $to);
	}

	//卸载插件
	public function uninstall(){
		global $ecs, $db;
		$table = $ecs->table('midoks_spider');
		$sql = "drop table {$table}";
		$res = $db->query($sql);

		//删除模版
		$to = M_EC_SPIDER_P_ROOT.'ec_midoks_spider_list.htm';
		@unlink($to);
		$to = M_EC_SPIDER_P_ROOT.'ec_midoks_spider_readme.htm';
		@unlink($to);
	}

	//菜单控制
	public function menu(){
		m_ec_menu(array(
			'menu' => array(
				'name'=>'EC蜘蛛抓取插件',//(必须)
				'file'=>'menu.php',//接收参数的文件(必须)
				'purview'=> 'ec_spider',//(可以不要了)
			),
			'submenu' => array(
				array('name' => '简要说明', 'link'=>'act=readme'),
				array('name' => '记录查看', 'link'=>'act=list'),	
			)
		),__FILE__);
	}

	//下面
	public function spiderList(){
		$spider_list = array (
			'googlebot' => '谷歌',
			'mediapartners-google' => 'Google Adsense',
			'baiduspider' => '百度',
			'bingbot' => '必应',
			'slurp' => '雅虎',
			'Sogou' => '搜狗',
			'sosospider' => '腾讯SOSO',
			'ia_archiver' => 'Alexa',
			'iaarchiver' => 'Alexa',
			'yodaobot' => 'Yodao',
			'sohu-search' => '搜狐',
			'msnbot' => 'MSN',
			'360Spider'=>'360',
			'DNSPod'=>'DNSPod',
			'JianKongBao'=> '监控宝',
			'YYSpider' => '云云搜索',
		);
		return $spider_list;
	}

	public function insertValue($spiderName, $url){
		$time = time();
		$ip = $this->get_client_ip();
		return $this->insertTableValue($spiderName, $time, $ip, $url);
	}

	//@func 向表插入值
	public function insertTableValue($spiderName, $time, $ip, $url){
		global $ecs, $db;
		$table = $ecs->table('midoks_spider');
		$sql = "insert into {$table}(id, name, time, ip, url) values(NULL, '{$spiderName}', '{$time}', '{$ip}', '{$url}')";
		return $db->query($sql);
	}

	//获取客服端真是IP
	public function get_client_ip(){
		static $ip = null;
		if($ip != null) return $ip;
		if( isset( $_SERVER['HTTP_X_FORWARDED_FOR'] ) ){
			$arr = explode( ',' ,$_SERVER['HTTP_X_FORWARDED_FOR'] );
			$pos = array_search( 'unknown' , $arr );
			if( false !=$pos ) unset($arr[$pos]);
			$ip = trim( $arr[0] );
		}elseif( isset( $_SERVER['HTTP_CLIENT_IP'] ) ){
			$ip = $_SERVER['HTTP_CLIENT_IP'];
		}elseif( isset($_SERVER['REMOTE_ADDR'] ) ){
			$ip = $_SERVER['REMOTE_ADDR'];
		}
		//检查IP地址的合法性
		$ip = (false!==ip2long($ip)) ? $ip : '0,0,0,0';
		return $ip;
	}

}
?>

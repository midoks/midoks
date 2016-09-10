<?php
/**
 * 	Description:返回顶部按钮
 *	Plugin Name:return-top
 *	Plugin URI:midoks.cachehca.com
 *	Author: midoks
 *	Version: 1.0
 */

class topreturn{

	/**
	 * @func 架构函数
	 */
	public function __construct(){
		
		//添加css
		add_action('wp_head', array($this, 'init_css'));
		//添加html
		add_action('wp_footer', array($this, 'add_html'));
	}

	public function init_css(){
		$url = plugin_dir_url(__FILE__);
		echo '<link type="text/css" rel="stylesheet" href="'.$url.'return_top.css" />';

		//添加js
		wp_register_script('return_top', plugins_url('return_top.js', __FILE__), '', '1.0', 'true');
		wp_enqueue_script('return_top');

	}

	public function add_html(){
		echo '<div><div class="midoks_returnTop"><span class="m"></span><span class="i"></span></div></div>';
	}	
}


//前台启用
//if (is_home()){
new topreturn();
//}
?>

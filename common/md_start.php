<?php


define('XHProf_Name', 'mdd');

/**
 * xhprof 配置,便于调试快速找到问题
 * author midoks@163.com
 */
function md_xhprof_start(){
	$root = "/Applications/mdserver/source";
	include_once $root.'/xhprof/xhprof_lib/utils/xhprof_lib.php';
	include_once $root.'/xhprof/xhprof_lib/utils/xhprof_runs.php';
	xhprof_enable();
}

function md_xhprof_end(){
	
  	//保存xhprof数据
  	$xhprof_data        = xhprof_disable();
	$xhprof_runs 		= new XHProfRuns_Default();

    $run_id = $xhprof_runs->save_run($xhprof_data, 'xhprof_foo');
 
    // url to the XHProf UI libraries (change the host name and path)
    $profiler_url = sprintf('http://localhost:8888/xhprof_html/index.php?run=%s&source=xhprof_foo', $run_id);
    //echo "<script language='javascript'><window.open('{$profiler_url}')</script>";
    echo '<a href="'. $profiler_url .'" target="_blank">Profiler output</a>';

}

if (( extension_loaded('md_xhprof') || extension_loaded('xhprof') ) 
	&& isset($_GET[XHProf_Name]) && $_GET[XHProf_Name] == 'ok' && 
	(!in_array($_SERVER['SCRIPT_NAME'], ['/xhprof_html/callgraph.php', '/xhprof_html/index.php']))){

	md_xhprof_start();
	include_once($_SERVER['SCRIPT_FILENAME']);
	md_xhprof_end();
	exit;
}
?>

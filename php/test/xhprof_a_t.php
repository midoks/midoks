<?php


function t_autoload($className){
	//var_dump($className);
	include_once($className.'.php');
	return false;
}

spl_autoload_register('t_autoload');


/**
 * xhprof 配置,便于调试快速找到问题
 * author midoks@163.com
 */
function t_xhprof_start(){
	if( extension_loaded('xhprof') ){
		$root = "/Applications/mdserver/source";
		include_once $root.'/xhprof/xhprof_lib/utils/xhprof_lib.php';
		include_once $root.'/xhprof/xhprof_lib/utils/xhprof_runs.php';
		xhprof_enable();
	}
}

function t_xhprof_end(){
	if( extension_loaded('xhprof') ){

	  	//保存xhprof数据
	  	$xhprof_data        = xhprof_disable();
		$xhprof_runs = new XHProfRuns_Default();

	    $run_id = $xhprof_runs->save_run($xhprof_data, 'xhprof_foo');
	 
	    // url to the XHProf UI libraries (change the host name and path)
	    $profiler_url = sprintf('http://localhost:8888/xhprof_html/index.php?run=%s&source=xhprof_foo', $run_id);
	    //echo "<script language='javascript'><window.open('{$profiler_url}')</script>";
	    echo '<a href="'. $profiler_url .'" target="_blank">Profiler output</a>';
	}
}


var_dump(extension_loaded('xhprof'));

t_xhprof_start();
$obj = new BB();
t_xhprof_end();


?>
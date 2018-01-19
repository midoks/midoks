<?php

//http://www.sinacloud.com/doc/sae/php/runtime.html#ying-yong-pei-zhi

$stderr = defined('STDERR') ? STDERR : fopen('php://stderr', 'w');


function E($msg){
	global $stderr;
	if(is_array($msg)){
		$msg = json_encode($msg);
	}
	fwrite($stderr, $msg);
	exit(1);
}

/**
 * /opt/csvn/bin/svnlook tree /opt/csvn/data/repositories/midoks /config.yaml
 */

$config = yaml_parse_file(__DIR__.'/config.yaml');

$runShellFile = $argv[0];

$projectDir = $argv[1];
// if (!file_exists($projectDir)){
// 	E($projectDir." no exist!!!");
// }

$svnDir = $argv[2];
if (!file_exists($svnDir)){
	E($projectDir." no exist!!!");
}

$svnVersion = $argv[3];
if (empty($svnDir)){
	E($svnVersion." no exist!!!");
}

$lastLine = exec("/opt/csvn/bin/svnlook cat $svnDir /config.yaml", $output, $retval);
if ($retval == 0){
	$projectConf = yaml_parse(implode("\r\n",$output));

	if (!$projectConf){
		E("A problem with config.yaml\r\n".implode("\r\n",$output));
	}

	$config = array_merge($config, $projectConf);
} 
else {
	exit(0);
}



$domain = $config['domain'];

// var_dump($config);
$content = '';
foreach ($domain as $v) {

	$tmp = explode(":", $v['name']);
	// var_dump($tmp);
	$serverName = $tmp[0];
	$port = $tmp[1];


	//server start
	$content .= "server {\r\n";

	$content .= "\tlisten {$port};\r\n";
	$content .= "\tserver_name {$serverName};\r\n";
	$content .= "\troot $projectDir;\r\n\r\n";
	$content .= "\tlocation / {\r\n";

	//index start
	$indexList = getConfigKey($config, 'nginx', 'index');
	$content .= "\t\tindex $indexList[0];\r\n";
	//index end


	//rewrite start
	$reList = getConfigKey($config, 'nginx', 'rewrite');
	$content .= "\t\tif (!-e \$request_filename){\r\n";
	foreach ($reList as $key => $value) {
		$content .= "\t\t\t$value;\r\n";
	}
	$content .= "\t\t}\r\n";
	//rewrite end

	$content .= "\t\tbreak;\r\n";
	$content .= "\t}\r\n";

	//php start
	$content .= "\tinclude php.conf;\r\n";
	//php end

	$content .= "}\r\n";
	//server end
}


$ret = file_put_contents('/usr/local/nginx/conf/vhost'."/tmp_".basename($projectDir).".conf", $content);
if (!$ret){
	E("Generation configuration failure");
}

exec("/usr/local/nginx/sbin/nginx -t", $output, $retval);
if ($retval != 0){
	E(implode("\r\n", $output));
}

exec("/usr/local/nginx/sbin/nginx -s reload", $output, $retval);
if ($retval != 0){
	E(implode("\r\n", $output));
}



function getConfigKey($config, $name, $key){
	$nameList = $config[$name];

	$list = array();
	foreach ($nameList as $k => $v) {
		foreach ($v as $k2 => $v2) {
			if ($k2 == $key){
				$list[] = $v2;
			}
		}
	}
	return $list;
}

?>
<?php 
/**
 * 权重算法
 */
$data   = array(
	array('id' => 1,  'name' => 'me',  'weight' => 5 ), 
	array('id' => 2,  'name' => 'we',  'weight' => 10 ), 
	array('id' => 3,  'name' => 'it',  'weight' => 15 ),
);


function weight_algo($list){

	$weight = 0;
	$tmp = array();

	foreach ($list as $value) {
		$weight += $value['weight'];

		for ($i=0; $i < $value['weight']; $i++) {
			$tmp[] = $value;
		}
	}

	$r = mt_rand(0, $weight-1);
	return $tmp[$r];
}


function weight_algo2(array $list) {

	$weight = array();
	foreach ($list as $k => $value) {
		$weight[$k] = $value['weight'];
	}

	$roll = rand(1, array_sum($weight));

	$tmpW = 0;
	foreach ($weight as $k => $v) {
		$min = $tmpW;
		$tmpW += $v;
		$max = $tmpW;
		if ($roll >= $min && $roll <= $max) {
			return $list[$k];
		}
	}
	return $list[$k];
}	

$rr = weight_algo($data);
var_dump('rr',$rr);

$rr2 = weight_algo2($data);
var_dump('rr2',$rr2);

?>
<?php

//
function R($start, $end = '', $dec = 3)
{
    static $_info = array();
    if (!empty($end)) {
        //
        if (!isset($_info['time'][$end]) && !isset($_info['mem'][$end])) {
            $_info['time'][$end] = microtime(true);
            $_info['mem'][$end] = memory_get_usage();
        }
        return array(
            'time' => number_format(($_info['time'][$end] - $_info['time'][$start]), $dec),
            'mem' => number_format(($_info['mem'][$end] - $_info['mem'][$start]), $dec),
        );
    } else {
        //
        $_info['time'][$start] = microtime(true);
        $_info['mem'][$start] = memory_get_usage();
    }
}

R('l');
for ($i = 0; $i < 1000000; $i++) {
    rand();
}
R('o');
$r = R('l', 'o');
echo $r['0'] . '::::::' . $r['1'];

echo '<br />';

R('1');
for ($i = 0; $i < 1000000; $i++) {
    mt_rand();
}
R('2');
$r = R('1', '2');
echo $r['0'] . '::::::' . $r['1'];

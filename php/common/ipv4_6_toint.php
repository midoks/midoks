
<?php

// 1
$ip = '202.203.44.225';
$ipToInt = sprintf('%u',ip2long($ip)); // 结果为：3402312929
echo "ip '202.203.44.225' to int is: ".$ipToInt;


// 2
$ipv6 = 'FEDC:BA98:7654:3210:FEDC:BA98:7654:3210';
$ip_n = inet_pton($ipv6);
$bits = 15; // 16 x 8 bit = 128bit
$ipv6long='';
while ($bits >= 0) {
        $bin = sprintf("%08b",(ord($ip_n[$bits])));
        $ipv6long = $bin.$ipv6long;
        $bits--;
}
echo gmp_strval(gmp_init($ipv6long,2),10); // 结果为：338770000845734292534325025077361652240

// 1 and 2
$ip = '192.168.101.100';
//$ip = 'FEDC:BA98:7654:3210:FEDC:BA98:7654:3210';
if(filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_IPV4)) {
    echo sprintf('%u',ip2long($ip));exit;
 
} else if(filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_IPV6)) {
 
$ip_n = inet_pton($ip);
$bits = 15; // 16 x 8 bit = 128bit
$ipv6long='';
    while ($bits >= 0) {
        $bin = sprintf("%08b",(ord($ip_n[$bits])));
        $ipv6long = $bin.$ipv6long;
        $bits--;
    }
    echo gmp_strval(gmp_init($ipv6long,2),10);exit;
}
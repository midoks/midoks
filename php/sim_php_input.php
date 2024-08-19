

<?php

// <?php
// $input_data = file_get_contents("php://input");
// var_dump($input_data);
//


function sim_php_input($url,$data){

    // var_dump($url,$data);

    $info = parse_url($url);

    $port = 80;
    if (isset($info['port'])){
        $port = $info['port'];
    }

    $http_entity_body = '';
    if (is_array($data)){
        $http_entity_body = json_encode($data);
    } else{
        $http_entity_body = $data;
    }

    // var_dump($http_entity_body);
    $http_entity_length = strlen($http_entity_body);
    $http_entity_type = 'application/x-www-form-urlencoded';
    $line = '';
    $fp = fsockopen($info["host"], $port,$error_no,$error_desc, 30);
    $inheader = 1; 
    if ($fp){
        fputs($fp, "POST {$info['path']} HTTP/1.1\r\n");
        fputs($fp, "Host: {$info["host"]}\r\n");
        fputs($fp, "Content-Type: {$http_entity_type}\r\n");
        fputs($fp, "Content-Length: {$http_entity_length}\r\n");
        fputs($fp, "Connection: close\r\n\r\n");
        fputs($fp, $http_entity_body . "\r\n\r\n");
 
        while (!feof($fp)) {
            $line .= fgets($fp, 4096);         
        }

        fclose($fp);
        $line = trim($line);
        $contentInfo = explode("\n\n", str_replace("\r", "", $line));
        $content = $contentInfo[1];
        if (!strstr($contentInfo[0], "HTTP/1.1 200 OK")) {
            return false;
        }

        $arr = explode("\n",$content);
        return trim($arr[1]);   
    }
    return $line;
}


$d = sim_php_input('http://test.xx:8000/input.php', ['order'=>'123']);
echo($d);
$d2 = sim_php_input('http://test.xx:8000/input.php', '{"task_id":"123","image":"https://xxx.com/xx.png","trans_data":"12"}');
echo($d2);
?>
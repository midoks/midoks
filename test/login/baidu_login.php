<?php

function ssl_get($url, $json = ''){
    $go = curl_init();
    curl_setopt($go, CURLOPT_URL, $url);
    curl_setopt($go, CURLOPT_FOLLOWLOCATION, 1);
    curl_setopt($go, CURLOPT_MAXREDIRS, 30);
    curl_setopt($go, CURLOPT_HEADER, 0);
    curl_setopt($go, CURLOPT_RETURNTRANSFER, 1);
    curl_setopt($go, CURLOPT_SSL_VERIFYPEER, false);
    curl_setopt($go, CURLOPT_TIMEOUT, 30);
    if(!empty($json)){//POST Data
        curl_setopt($go, CURLOPT_POST, 1);
        curl_setopt($go, CURLOPT_POSTFIELDS ,$json);
    }
    $response = curl_exec($go);
    curl_close($go);
    return $response;
}


function BD_start(){
    $appkey = 'RFfiWYPrPrGQktVRoD6GozeI';
    $reg_url = 'http://www.cachecha.com/login/baidu_login.php';
    $url = "http://openapi.baidu.com/oauth/2.0/authorize?response_type=code&client_id={$appkey}&redirect_uri={$reg_url}&scope=basic&display=popup";
    header('Location: '.$url);
}

function BD_start_2(){
    $appkey = 'RFfiWYPrPrGQktVRoD6GozeI';
    $appserect = '9pm6ONa1II0fgNAgQEzIAf8acfOOhUCg';
    $reg_url = 'http://www.cachecha.com/login/baidu_login.php';
    $code = $_GET['code'];
    $url = "https://openapi.baidu.com/oauth/2.0/token?";

    $post = "grant_type=authorization_code&code={$code}&client_id={$appkey}&client_secret={$appserect}&redirect_uri={$reg_url}";
    $data = ssl_get($url, $post);
    $data = json_decode($data, true);
    var_dump($data);


    $_t = BD_start_3($data['access_token'], $data);
    var_dump($_t);
}

function BD_start_3($token, $uid = ''){
    $url = "https://openapi.baidu.com/rest/2.0/passport/users/getInfo?access_token=".$token;
    if(!empty($uid)){
        $url .= '&uid='.$uid;
    }
    $data = file_get_contents($url);
    $data = json_decode($data, true);
    return $data;
}

if('1'==$_GET['bd_start']){//1
    BD_start();
}else if(isset($_GET['code'])){//2
    BD_start_2();
}else{
    echo "百度第一步:\r\n";
    echo '?bd_start=1';
}
?>

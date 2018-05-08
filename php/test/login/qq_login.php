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

function qq_start(){
    $appid = '100683516';
    $appserect = 'ad9c4d4d7588cb84b2b3025fdcc44d21';
    $reg_url = 'http://www.cachecha.com/login/qq_login.php';
    $reg_url = urlencode($reg_url);
    $_SESSION['state'] = md5(uniqid(rand(), TRUE)); 
    $scope = 'get_user_info';
    $url = "https://graph.qq.com/oauth2.0/authorize?response_type=code&client_id={$appid}&redirect_uri={$reg_url}&scope={$scope}&state={$_SESSION['state']}";

    echo("<script> top.location.href='" . $url . "'</script>");
    //header('Location: '.$url);
}


function qq_start_2(){
    $appid = '100683516';
    $appserect = 'ad9c4d4d7588cb84b2b3025fdcc44d21';
    $reg_url = 'http://www.cachecha.com/login/qq_login.php';
    $reg_url = urlencode($reg_url);
    $code = $_GET['code'];
    $url = "https://graph.qq.com/oauth2.0/token?grant_type=authorization_code&client_id={$appid}&client_secret={$appserect}&code={$code}&redirect_uri={$reg_url}";
    $data = file_get_contents($url);

    var_dump($data);
}


function qq_start_3(){
    $url = "https://graph.qq.com/user/get_user_info?format=json";
    $url .= '&access_token='.$token;
    $url .= '&oauth_consumer_key='.$key;
    $url .= '&openid='.$openid;
    return file_get_contents($url);
}

if('1'==$_GET['qq_start']){//1
    qq_start();
}else if(isset($_GET['code'])){//2
    qq_start_2();
}else{
    echo "qq第一步:\r\n";
    echo '?qq_start=1';
    echo '...';
}
?>

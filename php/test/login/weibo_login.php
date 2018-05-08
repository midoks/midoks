<?php



function weibo_start(){
    $appkey = '2491070417';
    $reg_url = 'http://www.cachecha.com/login/weibo_login.php';
    $url = "https://api.weibo.com/oauth2/authorize?client_id={$appkey}&response_type=code&redirect_uri={$reg_url}";
    header('Location: '.$url);
}

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


function weibo_start_2(){
    $appkey = '2491070417';
    $appserect = '03d5f7aa9c0c4679a343e104f7e3c706';
    $code = $_GET['code'];

    $reg_url = 'http://www.cachecha.com/login/weibo_login.php';
    $url = "https://api.weibo.com/oauth2/access_token";
    //echo $url."\r\n";
    $post = "client_id={$appkey}&client_secret={$appserect}&grant_type=authorization_code&redirect_uri={$reg_url}&code={$code}";
    $data = file_get_contents($url.$post);

    var_dump($data);
    
    
    
    
    //$data = ssl_get($url, $post);
    /*$data = json_decode($data, true);
    $token = $data['access_token'];
    $uid = $data['uid'];
    $_d = weibo_start_3($token, $uid);
    var_dump($_d);*/
}

function weibo_start_3($token, $uid){
    $url = "https://api.weibo.com/2/users/show.json?access_token={$token}&uid={$uid}";
    $content = file_get_contents($url);
    $content = json_decode($content, true);
    return $content;
}


if('1'==$_GET['weibo_start']){//1
    weibo_start();
}else if(isset($_GET['code'])){//2
    weibo_start_2();
}else{
    echo "weibo第一步:\r\n";
    echo '?weibo_start=1';
}
?>

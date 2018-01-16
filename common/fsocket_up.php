<?php

$url = 'https://api.weixin.qq.com/cgi-bin/media/upload?access_token=5_dDxEdba-jTqgCxGrevj1xDij4w4ugJ5SRLtWDdM64PndQT5_s5b7sofh9-4r5Pk8UXPJiGdPNk-NydyjeTD6qQlXungrHR0ObSOf3Nk9ox1EHTAGO7J1y0r1TsnJB7_xY4ilhGMx1Q7QTOgaQLOdABAMHK&type=image';

$filePath = dirname(__FILE__)."/wwq_1.jpg";

// $url = "http://test2.com/img_dl/t.php";

$ret = uploadImage($url, $filePath);
var_dump($ret);

function uploadImage($url, $filePath){

    $host   = parse_url($url, PHP_URL_HOST);
    $port   = parse_url($url, PHP_URL_PORT);
    $port   = $port ? $port : 80;
    $scheme = parse_url($url, PHP_URL_SCHEME);
    $path   = parse_url($url, PHP_URL_PATH);
    $query  = parse_url($url, PHP_URL_QUERY);

    if($query) {
        $path .= '?'.$query;
    }

    $port = 80;  
    $errno = '';  
    $errstr = '';  
    $timeout = 30;
      
    $form_data = array(  
        // 'name' => 'fdipzone',  
        // 'gender' => 'man',  
    );  
      
    $file_data = array(  
        array(  
            'name' => 'media',  
            'filename' => basename($filePath),
            'path' => $filePath 
        )
    );
      
    // create connect  
    $fp = fsockopen($host, $port, $errno, $errstr, $timeout);  
      
    if(!$fp){  
        return false;  
    }

    // send request  
    srand((double)microtime()*1000000);
    $boundary = "---------------------------".substr(md5(rand(0,32000)),0,10);
      
    $data = "--$boundary\r\n";  
      
    // form data  
    foreach($form_data as $key=>$val){  
        $data .= "Content-Disposition: form-data; name=\"".$key."\"\r\n";  
        $data .= "Content-type:text/plain\r\n\r\n";  
        $data .= rawurlencode($val)."\r\n";  
        $data .= "--$boundary\r\n";  
    }  
      
    // file data  
    foreach($file_data as $file){  
        $data .= "Content-Disposition: form-data; name=\"".$file['name']."\"; filename=\"".$file['filename']."\"\r\n";  
        $data .= "Content-Type: ".mime_content_type($file['path'])."\r\n\r\n";  
        $data .= implode("",file($file['path']))."\r\n";  
        $data .= "--$boundary\r\n";  
    }  
      
    $data .="--\r\n\r\n";  
      
    $out = "POST ${path} HTTP/1.1\r\n";  
    $out .= "Host:${host}\r\n";  
    $out .= "Content-type:multipart/form-data; boundary=$boundary\r\n"; // multipart/form-data  
    $out .= "Content-length:".strlen($data)."\r\n";  
    $out .= "Connection:close\r\n\r\n";  
    $out .= "${data}"; 

    // var_dump($out); 
    fputs($fp, $out);  
      
    // get response  
    $response = '';  
    while($row=fread($fp, 4096)){  
        $response .= $row;  
    }  
      
    fclose($fp);  
      
    $pos = strpos($response, "\r\n\r\n");  
    $response = substr($response, $pos+4);  
  
    return trim($response);
}

?>  
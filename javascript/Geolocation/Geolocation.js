/**
 *  你必须在服务器端(也可以是本地服务器)进行测试
 */
/**
getCurrentPosition() 方法 - 返回数据
若成功，则 getCurrentPosition() 方法返回对象。
始终会返回 latitude、longitude 以及 accuracy 属性。
如果可用，则会返回其他下面的属性。

属性	                描述
coords.latitude	        十进制数的纬度
coords.longitude	    十进制数的经度
coords.accuracy	        位置精度
coords.altitude	        海拔，海平面以上以米计
coords.altitudeAccuracy	位置的海拔精度
coords.heading	        方向，从正北开始以度计
coords.speed	        速度，以米/每秒计
timestamp	            响应的日期/时间
**/

//使用地址定位
function getLocation(c){
    if(navigator.geolocation){
        navigator.geolocation.getCurrentPosition(showPosition);
        return true;
    }else{
        return false;
        //console.log('Geolocation is not supported by this browser.');
    }
    //console.log(navigator.geolocation);
    function showPosition(pos){
        if(typeof c == 'function'){
            c(pos);
        }
    }
}


//使用地址定位(包含错误信息和拒绝)
function getLocationError(c){
    if(navigator.geolocation){
        navigator.geolocation.getCurrentPosition(showPosition, showError);
        return true;
    }else{
        return false;
    }
    
    //位置信息处理
    function showPosition(pos){
        if(typeof c == 'function'){
            c(pos);
        }
    }

    //错误处理
    function showError(error){
        console.log(error);
        /*switch(error.code){
            case error.PERMISSION_DENIED://用户不允许地址位置
                x.innerHTML="User denied the request for Geolocation."
                break;
            case error.POSITION_UNAVAILABLE://无法获取当前位置
                x.innerHTML="Location information is unavailable."
                break;
            case error.TIMEOUT://操作超时
                x.innerHTML="The request to get user location timed out."
                break;
            case error.UNKNOWN_ERROR://位置错误
                x.innerHTML="An unknown error occurred."
                break;
        }*/
    }
}


/*
watchPosition() - 返回用户的当前位置，并继续返回用户移动时的更新位置（就像汽车上的 GPS）。
clearWatch() - 停止 watchPosition() 方法
*/


//根据用户信息获取位置信息
function getLocationFllow(c){
    if(navigator.geolocation){
        navigator.geolocation.watchPosition(showPosition);
        return true;
    }else{
        return false;
    }
    
    //位置信息处理
    function showPosition(pos){
        if(typeof c == 'function'){
            c(pos);
        }
    }
}


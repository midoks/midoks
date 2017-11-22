arr = [ { a : 1, b : 2 }, { b : 1 }, { a : 1, b : 2, c : 3, d : 4 }, { a : 1 }, { a : 1, b : 2, c : 3 }, ];  
arr.sort(function(a1, a2) {  
    var i1 = 0,  
        i2 = 0;  
    for( x in a1) {  
        if( a1.hasOwnProperty(x) ) {  
            i1 += 1;   
        };  
    };  
    for( x in a2) {  
        if( a2.hasOwnProperty(x) ) {  
            i2 += 1;   
        };  
    };  
    return i1 - i2;  
}); 


console.log(JSON.stringify(arr)); //把对象转为JSON字符串  
//并未懂

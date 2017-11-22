/**
 *	@func 找出数组中最大的元素
 *	@param  array  查找的数组
 *	@return number 返回最大的数
 */
function find_max(num){
	var temp;
	for(var i=0,temp = num[i]; i<num.length; i++){
		if(i>0)
			temp = Math.max(temp, num[i]);
		else
			temp = Math.max(temp, num[i+1]),++i;
	}	
	return temp;
}




var a = [102,103,2,9,102,6,10,30,50,101];
console.log(find_max(a));

var b = Math.max.apply(null, a);
console.log(b);//更简便的方法

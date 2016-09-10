function print_fib(n){
	var arr = n >0 ? [1] : [];
	if(n > 1){
		for(var i = 1; i < n; i++)
			arr.push( arr[i-1] + ( i>=2 ? arr[i-2] : 0 ) );
	}
	//console.log(arr);
	return arr;
}

//test
console.log(print_fib(5));

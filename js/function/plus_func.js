function add(n1){
	return function(n2){
		return function(n3){
			return n1+n2+n3;
		};
	};
};

//test
console.log(add(2)(5)(5));

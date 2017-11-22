Number.prototype.plus = function(n){
	return this+n;
}

Number.prototype.minus = function(n){
	return this-n;
}

//test
console.log((5).plus(5).minus(2));

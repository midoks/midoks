(function($){
/**
 *	jquery扩展|tables操作(div+css操作)
 *	@param string 标签的id或class
 *	@param number 行
 *	@param number 列
 *	@param object div行的属性
 *	@param object div列的属性
 *	@return void
 */
$.fn.CreateTables = function(id, col, row, col_obj, row_obj){
	
	var tpl = "<div></div>", col_cmp, row_obj;
	for(e in col_obj){
		col_cmp = $(tpl).css(e, col_obj[e]).height(20);
	}

	//console.log(col_cmp[0].outerHTML);
	for(r in row_obj){
		row_obj = $(tpl).css(r, row_obj[r]).height(20);
	}

	console.log(row_obj);

	//行循环
	for(var i=0; i<col; i++){
		//console.log(i);
		var h = $(id).append(col_cmp[0].outerHTML);
		
		//console.log(h);

		

		//列循环
		for(var j=0; j<row; j++){

			$(h).append(row_obj[0].outerHTML);
			
		}
	
	}



}



})($);


/*	功能测试	*/
$.fn.CreateTables('#id', 6, 1, {color:'blue'}, {
	color:'red',
});

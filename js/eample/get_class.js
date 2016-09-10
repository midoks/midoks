//获取class对象
function get_class(ClassName){
	var tags = tags || document.getElementsByTagName('*');
	var list = [];
	for(var k in tags){
		var tag = tags[k];
		if(tag.className == ClassName){
			list.push(tag);
		}
	}
	return list;
}

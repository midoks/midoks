 var h51Time = window.setInterval(hidden51la,10); 
function hidden51la(){
    for(i=0;i<document.getElementsByTagName("a").length;i++){
        var temObj = document.getElementsByTagName("a")[i];
        if(temObj.href.indexOf("ajiang")>=0){
            temObj.style.display = "none";
		}
        if( temObj.href.indexOf("51.la")>=0){
            	temObj.style.display = "none";
            clearInterval(h51Time);
        }
    }
}

//////////////////////////////////////////////
 var h51Time = window.setInterval(hidden51la,10); 
function hidden51la(){
	var t = { a:'ajiang', a2:'51.la' }; 
    for(i=0;i<document.getElementsByTagName("a").length;i++){
        var temObj = document.getElementsByTagName("a")[i];
        if(temObj.href.indexOf(t.a)>=0){
            temObj.style.display = "none";
		}
        if( temObj.href.indexOf(t.a2)>=0){
            	temObj.style.display = "none";
            clearInterval(h51Time);
        }
    }
}

/////////////////////////eval压缩后
eval(function(p,a,c,k,e,r){e=function(c){return c.toString(36)};if('0'.replace(0,e)==0){while(c--)r[e(c)]=k[c];k=[function(e){return r[e]||e}];e=function(){return'[124-9b-f]'};c=1};while(c--)if(k[c])p=p.replace(new RegExp('\\b'+e(c)+'\\b','g'),k[c]);return p}('2 4=window.setInterval(5,10);function 5(){2 t={a:\\\'ajiang\\\',6:\\\'51.la\\\'};for(i=0;i<7.8("a").length;i++){2 1=7.8("a")[i];9(1.b.c(t.a)>=0){1.d.e="f"}9(1.b.c(t.6)>=0){1.d.e="f";clearInterval(4)}}}',[],16,'|temObj|var||h51Time|hidden51la|a2|document|getElementsByTagName|if||href|indexOf|style|display|none'.split('|'),0,{}))

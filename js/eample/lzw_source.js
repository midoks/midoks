function utf8(){
 
 }
 
 utf8.prototype.encode=function(text)
 {
     var result = "";
     for (var n = 0; n < text.length; n++)
     {
         var c = text.charCodeAt(n);
         if (c < 128)
         {
             result += String.fromCharCode(c);
         }
         else if (c > 127 && c < 2048)
         {
             result += String.fromCharCode((c >> 6) | 192);
             result += String.fromCharCode((c & 63) | 128);
         }
         else
         {
             result += String.fromCharCode((c >> 12) | 224);
             result += String.fromCharCode(((c >> 6) & 63) | 128);
             result += String.fromCharCode((c & 63) | 128);
         }
     }
     return result;
 }
 
 utf8.prototype.decode=function(text)
 {
     var result = "";
     var i = 0;
     var c1 = 0;
     var c2= 0;
     var c3 = 0;
     while (i < text.length)
     {
         c1 = text.charCodeAt(i);
         if (c1 < 128)
         {
             result += String.fromCharCode(c1);
             i++;
         }
         else if (c1 > 191 && c1 < 224)
         {
             c2 = text.charCodeAt(i + 1);
             result += String.fromCharCode(((c1 & 31) << 6) | (c2 & 63));
             i += 2;
         }
         else
         {
             c2 = text.charCodeAt(i + 1);
             c3 = text.charCodeAt(i + 2);
             result += String.fromCharCode(((c1 & 15) << 12) | ((c2 & 63) << 6) | (c3 & 63));
             i += 3;
         }
     }
     return result;
 }
 
 /**
  * Created with JetBrains WebStorm.
  * User: Administrator
  * Date: 12-8-3
  * Time: 下午4:01
  * To change this template use File | Settings | File Templates.
  */
 /**
  * lzw中文压缩类
  * @type {*}
  */
 function lzwcn(){
 
 }
 
 lzwcn.prototype.compress=function(str){
     var rStr='';
     rStr=as3long.utf8.encode(str);
     var i=0;
     var size=0;
     var xstr='';
     var chars = 256;
     var dict = new Array();
     for (i = 0; i < chars; i++)
     {
         dict[String(i)] = i;
     }
     var splitted=new Array();
     splitted   = rStr.split("");
     var buffer=new Array();
     size=splitted.length;
     var current='';
     var result = new String("");
     for(i = 0;i<=size;i++)
     {
         current = new String(splitted[i]);
         xstr = (buffer.length == 0) ? String(current.charCodeAt(0)) : (buffer.join("-") + "-" + String(current.charCodeAt(0)));
         if (dict[xstr] !== undefined)
         {
             buffer.push(current.charCodeAt(0));
         }
         else
         {
             result += String.fromCharCode(dict[buffer.join("-")]);
             dict[xstr] = chars;
             chars++;
             buffer = new Array();
             buffer.push(current.charCodeAt(0));
         }
     }
     return result;
 }
 
 lzwcn.prototype.decompress=function(str){
     var i;
     var chars = 256;
     var dict = new Array();
     for (i = 0; i < chars; i++)
     {
         dict[i] = String.fromCharCode(i);
     }
     var original = new String(str);
     var splitted= original.split("");
     var size = splitted.length;
     var buffer= new String("");
     var chain= new String("");
     var result = new String("");
     for (i = 0; i < size; i++)
     {
         var code = original.charCodeAt(i);
         var current = dict[code];
         if (buffer == "")
         {
             buffer = current;
             result += current;
         }
         else
         {
             if (code <= 255)
             {
                 result += current;
                 chain = buffer + current;
                 dict[chars] = chain;
                 chars++;
                 buffer = current;
             }
             else
             {
                 chain = dict[code];
                 if (chain == null)
                 {
                     chain = buffer + buffer.slice(0, 1);
                 }
                 result += chain;
                 dict[chars] = buffer + chain.slice(0, 1);
                 chars++;
                 buffer = chain;
             }
         }
     }
     result = as3long.utf8.decode(result);
     return result;
 }
 
 /**
  * Created with JetBrains WebStorm.
  * User: Administrator
  * Date: 12-8-3
  * Time: 下午4:50
  * To change this template use File | Settings | File Templates.
  */
 function as3long(){
 
 }
 
 as3long.prototype.lzwcn=new lzwcn();
 as3long.prototype.utf8=new utf8();
 window.as3long=new as3long();


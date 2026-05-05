
function showMsg(msg, callback ,icon, time){
	if (typeof time == 'undefined'){
		time = 2000;
	}

	if (typeof icon == 'undefined'){
		icon = {};
	}
	var loadT = layer.msg(msg, icon);
	setTimeout(function() {
		layer.close(loadT);
		if (typeof callback == 'function'){
			callback();
		}
	}, time);
}

function formatDate(t) {
    const now = new Date(t * 1000);
    var year = now.getFullYear();  //取得4位数的年份
    var month = now.getMonth() + 1;  //取得日期中的月份，其中0表示1月，11表示12月
    var date = now.getDate();      //返回日期月份中的天数（1到31）
    var hour = now.getHours();     //返回日期中的小时数（0到23）
    var minute = now.getMinutes(); //返回日期中的分钟数（0到59）
    var second = now.getSeconds(); //返回日期中的秒数（0到59）
    return year + "-" + month + "-" + date + " " + hour + ":" + minute + ":" + second;
}

//字符串转数组对象
function string2ArrayObject(str){
    var data = {};
    kv = str.split('&');
    for(i in kv){
        v = kv[i].split('=');
        data[v[0]] = v[1];
    }
    return data;
}

function copyText(value) {
    var clipboard = new ClipboardJS('#text_copys');
    clipboard.on('success', function (e) {
        layer.msg('复制成功',{icon:1,time:2000});
    });

    clipboard.on('error', function (e) {
        layer.msg('复制失败，浏览器不兼容!',{icon:2,time:2000});
    });
    $("#text_copys").attr('data-clipboard-text',value);
    $("#text_copys").click();
}

//表单多维转一维
function array2arr(sa){
    var t = {}

    for (var i = 0; i < sa.length; i++) {
        t[sa[i]['name']] = sa[i]['value'];
    }
    return t;
}

layui.config({
    base: '{__STATIC__}/admin/layuiadmin/'
}).use(['layer','form','element','jquery','table','laydate','util'],function() {
///
$ = layui.$;
layer = layui.layer;
form = layui.form;
element = layui.element;
table = layui.table;
laydate = layui.laydate;
util = layui.util;


//监听table表单搜索
form.on('submit(sreach)', function (data) {
	var _id = $(this).data('id');
    if (data.field.times) {
        var searchDate = data.field.times.split(' - ');
        data.field.kstime = searchDate[0];
        data.field.jstime = searchDate[1];
    } else {
        data.field.kstime = '';
        data.field.jstime = '';
    }
    data.field.times = undefined;
    var args = data.field;
    data.field['args'] = JSON.stringify(args);
    // console.log(data.field);
    table.reload(_id,{where: data.field, page:{curr: 1}});
});

//监听全局表单提交
form.on('submit(submit_save)', function(data){
    var index = layer.load();
    $.post(data.form.action, data.field, function(res) {
        console.log(res);
        showMsg(res.msg, function(){
            layer.close(index);
            if(res.code > -1){
                parent.location.reload();
            }
        },{icon: res.code > -1 ? 1 : 2,shift:res.code>-1 ? 0 : 6});
    },'json');
    return false;
});

// 时间范围选择
laydate.render({
    elem: 'input[name="times"]',
    type: 'datetime',
    range: true,
    rangeLinked: true,
    trigger: 'click'
});

var tps = '';
$('.layui-input,.layui-textarea').click(function(){
    if($(this).attr('placeholder') != tps){
        tps = $(this).attr('placeholder');
        if (tps && tps != ''){
            layer.tips(tps, $(this),{tips:1});
        }
    }
});

$('.table_more').click(function(){
    var target_class = $(this).attr('target_class');
    var status = $(this).data('status');

    if (status){
        $(this).data('status',false);
        $(this).find('cite').text("更多选项");
        $(this).find('i').removeClass("layui-icon-up").addClass('layui-icon-down');
        $("."+target_class).hide();
    } else{
        $(this).data('status',true);
        $(this).find('cite').text("收起选项");
        $(this).find('i').removeClass("layui-icon-down").addClass('layui-icon-up');

        $("."+target_class).show();
    }
});

///
});

;!function (win) {
///
"use strict";
var doc = document,
Admin = function(){
    this.v = '1.0'; //版本号
};
//默认加载
Admin.prototype.init = function () {
};

Admin.prototype.getRand = function(_id){
    var rand = Math.random().toString(36).substr(2)+Math.random().toString(36).substr(5);
    $('#'+_id).val(rand);
};

Admin.prototype.getPass = function(url){
    layer.prompt({title: '请输入新密码',area: ['200px', '150px']},function(value, index, elem){
        $.post(url, {pass:value}, function(res) {
            if(res.code == 1){
                layer.msg('修改成功',{icon: 1});
                setTimeout(function() {
                    location.reload();
                }, 1000);
            }else{
                layer.msg(res.msg,{icon: 2});
                layer.close(index);
            }
        },'json');
    });
}

//批量删除
Admin.prototype.batchDel = function(_url,_id) {
    var ids = [];
    if (isNaN(_id)) {
        var checkStatus = table.checkStatus(_id);
        checkStatus.data.forEach(function(n,i){
            ids.push(n.id);
        });
        var one = false;
    }else{
    	ids.push(_id);
    	var one = true;
    }

    if(ids.length == 0 ){
        layer.msg('请选择要删除的数据~!',{icon: 2,shift:6});
    }else{
        layer.confirm('确定要删除吗?', { title:'删除提示', btn: ['确定', '取消'],shade:0.001}, function(index) {
            $.post(_url, {'id':ids}, function(res) {
            	showMsg(res.msg, function(){
	        		if(res.code > -1){
	            		location.reload();
	           	 	}
	            },{icon: res.code > -1 ? 1 : 2,shift:res.code ? 0 : 6});
            },'json');
        }, function(index) {
            layer.close(index);
        });
    }
};

Admin.prototype.del = function(_this,_url,_id) {
    layer.confirm('确定要删除吗?', { title:'删除提示', btn: ['确定', '取消'],shade:0.001}, function(index) {
        $.post(_url, { 'id':_id }, function(res) {
            showMsg(res.msg, function(){
        		if(res.code > -1){
            		location.reload();
           	 	}
            },{icon: res.code > -1 ? 1 : 2,shift:res.code ? 0 : 6});
        },'json');
    }, function(index) {
        layer.close(index);
    });
};
//弹出层
Admin.prototype.open = function (title,url,w,h,full,data) {
    // console.log(title,url,w,h,full);
    if (title == null || title == '') {
        var title = false;
    };
    if (w == null || w == '') {
        var w = ($(window).width()*0.9);
    };
    if (h == null || h == '') {
        var h = ($(window).height() - 50);
    };
    h = h-20;
    
    // 存储数据到全局变量，供子页面访问
    if (data) {
        window._tempOpenData = data;
    }

    var open = layer.open({
        type: 2,
        area: [w+'px', h +'px'],
        fix: false, //不固定
        maxmin: true,
        shadeClose: false,
        maxmin: false,
        shade:0.2,
        title: title,
        content: url
    });
    if(full){
       layer.full(open);
    }
};
win.Admin = new Admin();
///
}(window);

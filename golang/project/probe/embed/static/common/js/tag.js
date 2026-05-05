var common_tags = {
    res:null,
    callback:null,
    url_related_list: '',
    url_tags_list: '',
    url_add:'',
    init: function(id, data){
        var _this = this;
        this.res = xmSelect.render({
            el: id,
            repeat: false,
            toolbar: {
                show: true,
                list: [ "ALL", "CLEAR",
                    {
                        icon: 'layui-icon layui-icon-face-smile',
                        name: '添加标签',
                        method: function(data){
                            Admin.open('新增标签',_this.url_add, 500,315);
                        }
                    } 
                ]
            },
            filterable: true,
            remoteSearch: true, 
            remoteMethod: function(val, cb, show){
                var args = {};
                args['status'] = '1';
                args['page'] = '1';
                args['limit'] = '5';
                args['zd'] = 'name';
                args['key'] = val;
                // console.log(_this.url_tags_list);
                $.get(_this.url_tags_list, args, function(res) {
                    var tags_list = [];
                    var list = res['data'];
                    for (var i = 0; i < list.length; i++) {
                        var t = {};
                        t['name'] = list[i]['ip'];
                        t['value'] = list[i]['id'];
                        tags_list.push(t);
                    }
                    cb(tags_list);
                },'json');
            },
            paging: true,
            pageSize: 3,
            data: data,
            on: function(data){
                setTimeout(function(){
                    common_tags.cb();
                },10);
            },
        });
    },

    initData: function(id, server_ids, callback){
        this.callback = callback;
        var _this = this;
        var args = {};
        args['status'] = '1';
        args['page'] = '1';
        args['limit'] = '1000';

        var server_arr = server_ids.split(",");
        $.get(this.url_related_list, args, function(res) {
            var tags_list = [];
            var list = res['data'];

            // console.log(list);
            for (var i = 0; i < list.length; i++) {
                var vvid = list[i]['id'];
                var is_select = false;
                console.log("vvid", vvid);

                for (var j = 0; j < server_arr.length; j++) {
                    var ssid = server_arr[j];
                    console.log("ssid",ssid);
                    if (ssid == vvid){
                        is_select = true;
                    }
                }

                var t = {};
                t['name'] = list[i]['ip'];
                t['value'] = list[i]['id'];

                if (is_select){
                    t['selected'] = true;
                } else {
                    t['selected'] = false;
                }
                tags_list.push(t);
            }
            _this.init(id, tags_list);
            _this.cb();
        },'json'); 
    },

    cb: function(){
        // console.log(this.res);
        var v = this.res.getValue('value');
        if (this.callback){
            this.callback(v);
        }
    }
};
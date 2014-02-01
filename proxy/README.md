《在openshift配置的proxy》
==========================

这是我在使用到了www.openshift.com的网站提供的应用,绑定我的域名。用这个服务器获取我BAE上的内容,感觉好复杂!!!

### 基本信息

- **文件缓存**：我做了一下文件缓存
- **计划任务**：进入 /var/lib/openshift/{youcode}/app-root/repo/.openshift/cron,里面有{daily  hourly  minutely  monthly weekly}等,修改del.sh中的有{youcode},上传到daily文件或其他的就可以了。
- **参考1**：[新OpenShift免费空间申请使用教程:绑定域名,文件管理和安装程序(http://www.freehao123.com/openshift-wp-dz/)]

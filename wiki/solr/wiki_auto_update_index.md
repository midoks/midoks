# Solr自动更新或重建索引

- 无效
- [apache-solr-dataimportscheduler-1.0.jar](conf/apache-solr-dataimportscheduler-1.0.jar)放在WEB-INF/lib.

- 修改WEB-INF/web.xml
```
<listener>  
 <listener-class>  
         org.apache.solr.handler.dataimport.scheduler.ApplicationListener  
 </listener-class>  
</listener>
```


- 简单使用
```
0,30 * * * * /usr/bin/wget http://<solr_host>:8983/solr/<core_name>/dataimport?command=full-import
```
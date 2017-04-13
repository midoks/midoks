# Solr服务
Solr是一个独立的企业级搜索应用服务器，它对外提供类似于Web-service的API接口。用户可以通过http请求，向搜索引擎服务器提交一定格式的XML文件，生成索引；也可以通过Http Get操作提出查找请求，并得到XML(及其他格式)的返回结果。

# 相关文档
- http://www.apache.org/dyn/closer.lua/lucene/solr/6.3.0
- https://lucene.apache.org/solr/resources.html#tutorials
- http://dev.mysql.com/downloads/connector/j/
- https://wiki.apache.org/solr/DataImportHandler
- https://github.com/mbonaci/solr-data-import-scheduler[scheduler组件]

#安装solr准备工作
* JAVA环境安装
* windows -> solr-6.3.0.zip
* Unix,Linux,MacOSX -> solr-6.3.0.tgz

# 常用命令
* bin/solr start
* bin/solr stop
* bin/solr restart

# 配置
```
./bin/solr create -c core
```

- MySQL数据为例
```
CREATE TABLE `test1` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(200) NOT NULL,
  `value` text NOT NULL,
  `time` int(11) NOT NULL,
  `isdel` tinyint(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4;
```

- 复制solrconfig.xml文件(server/solr/configsets/basic_configs/conf)

```
<requestHandler name="/dataimport" class="solr.DataImportHandler">  
     <lst name="defaults">  
        <str name="config">db-data-config.xml</str>  
     </lst>  
</requestHandler>
```
- 创建(db-data-config.xml)文件

做好带有时间字段
```
<dataConfig>
    <dataSource driver="com.mysql.jdbc.Driver" url="jdbc:mysql://127.0.0.1:3306/test" user="root" password="root"/>
    <document>
    	<!-- 
			query 				| 获取全部数据的SQL
			deltaImportQuery 	| 是获取增量数据时使用的SQL 
			deltaQuery 			| 是获取pk的SQL
			parentDeltaQuery	| 是获取父Entity的pk的SQL
			deletedPkQuery 		| 增量索引删除主键ID查询
    	-->
    <entity name="test"
        pk="id"       
        query="select * from test1"
        deltaImportQuery="select * from test1 where id='${dih.delta.id}'"
        deltaQuery="select id from test1 where FROM_UNIXTIME(`time`,'%Y-%m-%d %H:%i:%s')>'${dih.last_index_time}'"
        deletedPkQuery="select id from test1 where FROM_UNIXTIME(`time`,'%Y-%m-%d %H:%i:%s')>'${dih.last_index_time}'"

        	>
            <field column="id" name="id" />
            <field column="name" name="name" />
            <field column="value" name="value" />
    </entity>
    </document>
</dataConfig>
```

- managed-schema
```
solr6.0后默认managed-schema,可在后台编辑添加
```

- 创建(core.properties)文件
```
echo name=mysql_test > core.properties
```

- 查看dataimport.properties参数说明[scheduler组件解决方法](http://blog.csdn.net/yxue1118/article/details/51800145)|[查看](dataimport.properties.md)

- 8小时时差问题(bin/solr.in.sh)
```
SOLR_TIMEZONE="UTC+8"
```

- solr集群搭建([查看文档](wiki_solr_cloud.md))








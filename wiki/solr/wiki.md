# Solr服务
Solr是一个独立的企业级搜索应用服务器，它对外提供类似于Web-service的API接口。用户可以通过http请求，向搜索引擎服务器提交一定格式的XML文件，生成索引；也可以通过Http Get操作提出查找请求，并得到XML(及其他格式)的返回结果。

# 相关文档
- http://www.apache.org/dyn/closer.lua/lucene/solr/6.3.0
- https://lucene.apache.org/solr/resources.html#tutorials
- http://dev.mysql.com/downloads/connector/j/
```
mysql-connector-java-5.1.41-bin.jar 
```
- https://wiki.apache.org/solr/DataImportHandler
- https://github.com/mbonaci/solr-data-import-scheduler[scheduler组件]

#安装solr准备工作
* JAVA环境安装()
* windows -> solr-6.3.0.zip
* Unix,Linux,MacOSX -> solr-6.3.0.tgz

# 常用命令
* bin/solr start
* bin/solr stop
* bin/solr restart
* /usr/local/src/solr-6.5.0/bin/solr restart

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

- 添加库
```
<lib dir="${solr.install.dir:../../../..}/dist/" regex="mysql-connector-java-5.1.40-bin.jar" />
<lib dir="${solr.install.dir:../../../..}/dist/" regex="solr-dataimporthandler-.*\.jar" />
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
- solr分词([查看文档](wiki_solr_participle.md))




# 主solr配置，修改配置文件
```
<requestHandler name="/replication" class="solr.ReplicationHandler" >
  <lst name="master">
   <str name="replicateAfter">commit</str>
   <str name="replicateAfter">startup</str>
   <str name="confFiles">schema.xml,stopwords.txt</str>
  </lst>
</requestHandler>
```
 

# 从solr配置,修改配置文件
```
<requestHandler name="/replication" class="solr.ReplicationHandler" >
 <lst name="slave">
   <str name="masterUrl">http://127.0.0.1/core0/replication</str><!--主搜索引擎服务地址-->
   <str name="pollInterval">00:00:60</str><!--同步频率，1分钟一次-->
 </lst>
</requestHandler>

masterUrl : 主服务器同步URL地址 
pollInterval:从服务器同步间隔，即每隔多长时间同步一次主服务器 
httpConnTimeout:设置连接超时（单位：毫秒） 
httpReadTimeout:如果设置同步索引文件过大，则应适当提高此值。（单位：毫秒） 
httpBasicAuthUser:验证用户名，需要和主服务器一致 
httpBasicAuthPassword:验证密码，需和主服务器一致 
compression:external or internal 使用SOLR自己的压缩算法或应用容器的
```

# 用户权限配置
- http://wiki.eclipse.org/Jetty/Tutorial/Realms

- 生成密码
```
在/solr/server/etc/目录下[示例文件](conf/realm.properties)

java -cp server/lib/jetty-util-9.3.14.v20161028.jar org.eclipse.jetty.util.security.Password admin admin
admin
OBF:1u2a1toa1w8v1tok1u30
MD5:21232f297a57a5a743894a0e4a801fc3
CRYPT:adpexzg3FUZAk
```

- 在/server/contexts/solr-jetty-context.xml中添加内容([示例文件](conf/solr-jetty-context.xml))

```
<Get name="securityHandler">  
 <Set name="loginService">  
   <New class="org.eclipse.jetty.security.HashLoginService">  
    <Set name="name">Test Reaml</Set>  
    <Set name="config"><SystemProperty name="jetty.home" default="."/>/etc/realm.properties</Set>  
   </New>  
 </Set>  
</Get>
```

- 在server/solr-webapp/webapp/WEB-INF/web.xml中添加如下内容([示例文件](conf/web.xml))
```
<security-constraint>
    <web-resource-collection>
      <web-resource-name>solr</web-resource-name>
      <url-pattern>/</url-pattern>
    </web-resource-collection>
    <auth-constraint> 
        <role-name>solr_home</role-name>  
        <role-name>admin</role-name>  
    </auth-constraint>
  </security-constraint>
  <login-config>  
      <auth-method>BASIC</auth-method>  
      <realm-name>Solr</realm-name>  
  </login-config>  
```


# 常用操作

- 清空数据
```
在solr客户端，访问你的索引库（我认为最方便的方法）
documents type 选择 XML 
documents 输入下面语句

<delete><query>*:*</query></delete>
<commit/>
```
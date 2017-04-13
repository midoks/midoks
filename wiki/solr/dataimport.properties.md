# dataimport.properties 配置说明
```
#################################################
#                                               #
#       dataimport scheduler properties         #
#                                               #
#################################################

#  to sync or not to sync
#  1 - active; anything else - inactive
syncEnabled=1

#  which cores to schedule
#  in a multi-core environment you can decide which cores you want syncronized
#  leave empty or comment it out if using single-core deployment
#syncCores=game,resource
syncCores=collection1
#  solr server name or IP address
#  [defaults to localhost if empty]
server=localhost

#  solr server port
#  [defaults to 80 if empty]
port=8080

#  application name/context
#  [defaults to current ServletContextListener's context (app) name]
webapp=solr

#  URL params [mandatory]
#  remainder of URL
params=/dataimport?command=delta-import&clean=false&commit=true

#  schedule interval
#  number of minutes between two runs
#  [defaults to 30 if empty]
interval=1

#  重做索引的时间间隔，单位分钟，默认7200，即1天; 
#  为空,为0,或者注释掉:表示永不重做索引
reBuildIndexInterval=2

#  重做索引的参数
reBuildIndexParams=/dataimport?command=full-import&clean=true&commit=true

#  重做索引时间间隔的计时开始时间，第一次真正执行的时间=reBuildIndexBeginTime+reBuildIndexInterval*60*1000；
#  两种格式：2012-04-11 03:10:00 或者  03:10:00，后一种会自动补全日期部分为服务启动时的日期
reBuildIndexBeginTime=03:10:00
```
### php调式
```
1.修改php-fpm.conf中配置 没有则增加
error_log = log/error_log

2.修改php.ini中配置，没有则增加
log_errors = On
error_log = "/usr/local/php71/var/log/error_log"
error_reporting=E_ALL&~E_NOTICE
```        
# mysql 定时触发器

# 创建表
```
CREATE TABLE `t1` (  
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `val` varchar(12) COLLATE utf8_bin NOT NULL,
  `add_time` int(11) COLLATE utf8_bin NOT NULL,
  `up_time` int(11) COLLATE utf8_bin NOT NULL,
  PRIMARY KEY (`id`)  
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8 COLLATE=utf8_bin; 
```

# 创建存储过程
```
DELIMITER $$  
  
--  
-- 存储过程  
--  
CREATE PROCEDURE `t1_del_count`(IN `date_inter` INT)
BEGIN  
    DELETE FROM t1 WHERE (TO_DAYS(NOW()) - TO_DAYS(FROM_UNIXTIME(`up_time`))>=date_inter;
END$$  
DELIMITER ; 
```

# 创建定时任务
```
CREATE EVENT `e_del_tbl_base_count`   
ON SCHEDULE EVERY 1 DAY STARTS '2012-12-1 24:00:00'   
ON COMPLETION NOT PRESERVE ENABLE DO CALL p_del_count (90);
```
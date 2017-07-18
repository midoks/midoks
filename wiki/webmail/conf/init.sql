use extmail;

/*!40000 ALTER TABLE `alias` DISABLE KEYS */;
LOCK TABLES `alias` WRITE;
INSERT INTO `alias` VALUES ('support@extmail.org','postmaster@extmail.org','extmail.org','2007-02-14 15:10:04',1);
UNLOCK TABLES;
/*!40000 ALTER TABLE `alias` ENABLE KEYS */;

/*!40000 ALTER TABLE `domain` DISABLE KEYS */;
LOCK TABLES `domain` WRITE;
INSERT INTO `domain` VALUES ('extmail.org','virtualDomain for extmail.org','A0/B0',50,50,1073741824,1073741824,NULL,'1','5242880','5242880','1y','0','0','0','0','1','0','2007-02-14 15:10:04','2010-11-08',1);
UNLOCK TABLES;
/*!40000 ALTER TABLE `domain` ENABLE KEYS */;

/*!40000 ALTER TABLE `mailbox` DISABLE KEYS */;
/* postmaster@extmail.org password is extmail */
LOCK TABLES `mailbox` WRITE;
INSERT INTO `mailbox` VALUES ('postmaster@extmail.org','postmaster','$1$phz1mRrj$3ok6BjeaoJYWDBsEPZb5C0','','PostMaster','','extmail.org/postmaster/Maildir/','extmail.org/postmaster','104857600S','52428800S','extmail.org',1000,1000,'2007-02-14 15:10:04','2010-11-08',1,0,0,0,0,0,0,0,'my question','my answer');
UNLOCK TABLES;
/*!40000 ALTER TABLE `mailbox` ENABLE KEYS */;

/*!40000 ALTER TABLE `manager` DISABLE KEYS */;
/* root@extmail.org password is extmail*123* */
LOCK TABLES `manager` WRITE;
INSERT INTO `manager` VALUES ('root@extmail.org','$1$BrT9qxfB$Ha81Mb5YVV6rNKNN5jmtj1','admin','root','Super User','my question','my answer','0','2007-02-14 15:10:04','2010-11-08',1);
UNLOCK TABLES;
/*!40000 ALTER TABLE `manager` ENABLE KEYS */;

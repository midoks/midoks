/*
 ExtMail - OSS
 by He zhiqiang <hzqbbc@hzqbbc.com>
 Copyright (c) 1998-2005 hzqbbc.com
 License: GPL v2


 This is the MySQL database structure for ExtMail System, derive from
 Postfixadmin ( http://high5.net/howto/ ) project, and modify to meet
 our need, it is compatible with PostfixAdmin MySQL structure and have
 some extmail specific attributes and structure

 ExtMail have an alternative solution on openLDAP, please check the
 extmail.schema (V3) for detail.

 You can create the database from the shell with:

 mysql -u root [-p] < extmail.sql
*/

/*
 Initialize mysql
*/

USE mysql;
/* readonly user & password */
/* clean user if exists */
delete FROM user where User in ('extmail','webman');
delete FROM db where User in ('extmail','webman');

INSERT INTO user (Host, User, Password) VALUES ('localhost','extmail',password('extmail'));
INSERT INTO db (Host, Db, User, Select_priv) VALUES ('localhost','extmail','extmail','Y');

/* extman read/write user & password */
INSERT INTO user (Host, User, Password) VALUES ('localhost','webman',password('webman'));
INSERT INTO db (Host, Db, User, Select_priv, Insert_priv, Update_priv, Delete_priv) VALUES ('localhost', 'extmail', 'webman', 'Y', 'Y', 'Y', 'Y');
FLUSH PRIVILEGES;

/* readonly user */
GRANT USAGE ON extmail.* TO extmail@localhost;
GRANT SELECT, UPDATE ON extmail.* TO extmail@localhost;

/* read/write user */
GRANT USAGE ON extmail.* TO webman@localhost;
GRANT SELECT, INSERT, DELETE, UPDATE ON extmail.* TO webman@localhost;
CREATE DATABASE extmail;
USE extmail;

USE extmail;

/* Table structure for table manager */
CREATE TABLE manager (
  username varchar(200) NOT NULL default '',
  `password` varchar(255) NOT NULL default '',
  `type` varchar(64) NOT NULL default 'postmaster',
  uid varchar(255) NOT NULL default '',
  name varchar(255) NOT NULL default '',
  question text NOT NULL default '',
  answer text NOT NULL default '',
  disablepwdchange smallint(1),
  createdate datetime NOT NULL default '0000-00-00 00:00:00',
  expiredate DATE NOT NULL default '0000-00-00',
  active tinyint(1) NOT NULL default '1',
  PRIMARY KEY  (username),
  KEY username (username)
) ENGINE=MyISAM;

/* Table structure for table alias */
CREATE TABLE alias (
  address varchar(200) NOT NULL default '',
  goto text NOT NULL,
  domain varchar(255) NOT NULL default '',
  createdate datetime NOT NULL default '0000-00-00 00:00:00',
  active tinyint(1) NOT NULL default '1',
  PRIMARY KEY  (address),
  KEY address (address)
) ENGINE=MyISAM COMMENT='ExtMail - Virtual Aliases';

/* Table structure for table domain */
CREATE TABLE domain (
  domain varchar(200) NOT NULL default '',
  description varchar(255) NOT NULL default '',
  hashdirpath varchar(255) NOT NULL default '',
  maxalias int(10) NOT NULL default '0',
  maxusers int(10) NOT NULL default '0',
  maxquota varchar(16) NOT NULL default '0',
  maxnetdiskquota varchar(16) NOT NULL default '0',
  transport varchar(255) default NULL,
  can_signup tinyint(1) NOT NULL default '0',
  default_quota varchar(255) default NULL,
  default_netdiskquota varchar(255) default NULL,
  default_expire varchar(12) default NULL,
  disablesmtpd smallint(1),
  disablesmtp smallint(1),
  disablewebmail smallint(1),
  disablenetdisk smallint(1),
  disableimap smallint(1),
  disablepop3 smallint(1),
  createdate datetime NOT NULL default '0000-00-00 00:00:00',
  expiredate DATE NOT NULL default '0000-00-00',
  active tinyint(1) NOT NULL default '1',
  PRIMARY KEY  (domain),
  KEY domain (domain)
) ENGINE=MyISAM COMMENT='ExtMail - Virtual Domains';

/* Table structure for table domain_manager */
CREATE TABLE domain_manager (
  username varchar(200) NOT NULL default '',
  domain varchar(255) NOT NULL default '',
  createdate datetime NOT NULL default '0000-00-00 00:00:00',
  active tinyint(1) NOT NULL default '1',
  KEY username (username)
) ENGINE=MyISAM COMMENT='Ext/Webman - Domain Admins';

/*
 Table structure for table mailbox
 mapping: name <-> cn, username <-> mail
*/
CREATE TABLE mailbox (
  username varchar(200) NOT NULL default '',
  uid varchar(255) NOT NULL default '',
  password varchar(255) NOT NULL default '',
  clearpwd varchar(128) NOT NULL default '',
  name varchar(255) NOT NULL default '',
  mailhost varchar(255) NOT NULL default '',
  maildir varchar(255) NOT NULL default '',
  homedir varchar(255) NOT NULL default '',
  quota varchar(16) NOT NULL default '0',
  netdiskquota varchar(16) NOT NULL default '0',
  domain varchar(255) NOT NULL default '',
  uidnumber int(6) NOT NULL default '1000',
  gidnumber int(6) NOT NULL default '1000',
  createdate datetime NOT NULL default '0000-00-00 00:00:00',
  expiredate DATE NOT NULL default '0000-00-00',
  active smallint(1) NOT NULL default '1',
  disablepwdchange smallint(1),
  disablesmtpd smallint(1),
  disablesmtp smallint(1),
  disablewebmail smallint(1),
  disablenetdisk smallint(1),
  disableimap smallint(1),
  disablepop3 smallint(1),
  question text NOT NULL default '',
  answer text NOT NULL default '',
  PRIMARY KEY  (username),
  KEY username (username)
) ENGINE=MyISAM COMMENT='ExtMail - Virtual Mailboxes';

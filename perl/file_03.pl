#!/usr/bin/perl

use 5.010;

# perl file_03.pl


print "hello\n";


# -e 测试文件是否存在
# -M 测试文件存在多少天
# -s 测试文件大小
# -A 测试文件大小
# -z 测试文件是否是空的
# -f 测试文件是否存在
# -d 测试目录是否存在
# -r 测试文件可读

print "file is readable\n" if -r "file_03.pl";
print "file is writeable\n" if -w "file_03.pl";
print "file is a regular file\n" if -f "file_03.pl";
print "file is a dir file\n" if -d "file_03.pl";
print "file size is:",  -s "file_03.pl","\n";


my($dev ,$inode, $mode,  $nlink, $uid, $gid, $rdev, $size, $atime, $mtime, $ctime, $bksize, $blocks)= stat("file_03.pl");


print $dev ,"\n";




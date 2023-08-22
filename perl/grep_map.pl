#!/usr/bin/perl

# perl grep_map.pl

# my @odd_numbers;
# foreach  (1..1000) {
# 	push @odd_numbers, $_ if $_ % 2;
# }
# print "@odd_numbers\n";


# 
# eval {
# 	print "\n\ngrep..\n";
#     my @odd=grep $_ % 2, 1..10;
# 	print "@odd\n";
# };
# if ($@) {
# 	print "$@";
# }


# 文件操作 <> 代表输入文件
# my @matching_lines = grep /do_someting/i, <>;
# print "@matching_lines";

# 循环读取文件名
my @file = glob "*.*";
print "@file\n";


# 提取不带后缀的文件名
my @txx = map { /(.*)\.pl$/ } @file;
print "@txx\n";

# 获取文件大小
my @sizes = map -s, @file;
print "@sizes\n";


foreach(1..10) {
     print "I can count $_! \n";
}







#!/usr/bin/perl

use 5.010;

# https://www.runoob.com/perl/perl-regular-expressions.html
# perl reg_02.pl

print "hello\n";

$_ = "Hello there, neighbor";

if (/\s(\w+),/){ 
	print "$1\n";
}


if (/(\S+) (\S+), (\S+)/){ 
	print "$1 $2 $3\n";
}


my $diao = "i fear that i'll be extinct after 1000 years.";
if ($diao =~ /(\d*) years/){ 
	print "$1\n";
}


$_ = "one two three four five six";
if (/(\S+) (\S+) (\S+)/){ 
	print "$1 $2 $3\n";
}


my $names = "fred and barney";
if ($names =~ m/(\w+) (?:and|or) (\w+)/){ 
	print "$1,$2\n";
}

if ($names =~ m/(?<name1>\w+) (?:and|or) (?<name2>\w+)/){ 
	print "$+{name1},$+{name2}\n";
}


$_ = "one two three, four five six";
print "------\n";
# $`: 匹配部分的前一部分字符串
# $&: 匹配的字符串
# $': 还没有匹配的剩余字符串
if (/\s(\w+),/){ 
	print "$1\n";
	print "$&\n";
	print "$'\n";
	print "$`\n";
	print "$`$&$`\n";
	print "$`<$&>$`\n";
}



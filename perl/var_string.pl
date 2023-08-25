#!/usr/bin/perl

# perl var_string.pl
# print "\a\a";


print "飞船操作符\n";


print "input one:";
chomp( $num1 = <STDIN> );

print "input two:";
chomp( $num2 = <STDIN> );


$result = $num1 <=> $num2;

print "result: $result\n";

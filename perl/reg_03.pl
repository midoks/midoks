#!/usr/bin/perl

use 5.010;

# https://www.runoob.com/perl/perl-regular-expressions.html
# perl reg_03.pl


$_ = "he's out bowing with barney tonight";

s/barney/red/;
print "$_\n";

s/with (\w+)/against $1's team/;
print "$_\n";


$_ = "one two three";
s/(\w+) (\w+)/$2,$1/;
print "$_\n";


s/^/huge,/;
print "$_\n";

$_ = "home two home";
s/home/cave/g;
print "$_\n";


$_ = "Input    data\t may have     extra whitespace.";
s/\s+/ /g;
print "$_\n";


$_ = "               one two three    ";
# s/^\s+//g;
# s/\s+$//g;
s/^\s+|\s+$//g;
print "$_\n";



my $ccc = "one another one";
$ccc =~ s/one/two/g;
print "$ccc\n";

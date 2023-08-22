#!/usr/bin/perl

# perl var_scalar.pl


print "123rrr123" * "3";
print "\n";
print "----\n";
print "ff" * 3;
print "\n";
print "hello" . 8;
print "\n";
print "hello" . 8*6;
print "\n";

$fred = 8+6;
print "hello" . $fred;
print "\n";
print "----\n";

$fred2 = 6**2;
print "hello" . $fred2;
print "\n";
print "----\n";
$_hello = 99;
print  $_hello;
print "\n";

print "----\n";
# $9a = 11;
# print  $9a;
$fred = 5;
$fred = $fred+5;
$fred += 5;
print "hello:" . $fred;
print "\n";

print "----\n";

$barney = 17;
$barney *= 3;
print $barney;
print "\n";



print "----\n";

$str = "hello";
$str .= "  ";
$str .= "Perl";
print $str;

print "----\n";
print "The answer is ". 6*7 . "\n";
print "----\n";


$a = 12;
$b = 16;

if ($a == $b ){
	print "eq\n";
} else {
	print "not eq\n";
}









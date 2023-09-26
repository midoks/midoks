#!/usr/bin/perl

use 5.010;
# perl sort_01.pl

print "hello\n";


my @rocks = qw/ bedrock slate rubble granite /;
my @sorted = sort(@rocks);

print "@sorted\n";

my @numbers = (20,1,10,2);
my @result = sort @numbers;
print "@result\n";

my @result = sort { $a <=> $b } @numbers;
print "@result\n";

sub by_num{
	# my($a,$b) = @_;
	# if ($a<$b){return -1;} elsif($a>$b){1} else{0}
	$a <=> $b
}

my @result  = sort by_num @numbers;
print "@result\n";



my @result = sort qw( cat apple big cat);
print "@result\n";

my @result = sort { "\L$a" cmp "\L$b" } qw( cat apple big Cat);
print "@result\n";


my %el = (B=>5, Be=>4,H=>1,He=>2,Li=>3);
my @result = sort keys %el;
print "@result\n";

my @result = sort { $el{$b} <=> $el{$c} } keys %el;
print "@result\n";

$m = "Apple";
$n = "Dog";
print $m cmp $n;
print "\n";



$m = "ZApple";
$n = "Dog";
print $m cmp $n;
print "\n";

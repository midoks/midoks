#!/usr/bin/perl

use 5.010;
# perl sub_02.pl

print "hello\n";

foreach  (1..10) {
	my($square) = $_*$_;
	print "$_ square is $square\n";
}


sub demo{
	my ($num1) = @_;
	my $num2 = @_;

	print "$num1\n";
	print "$num2\n";
}

&demo(8,9,10);


$bammbamm += 1;
print "$bammbamm\n";
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




sub find{
	my ($what, @array) = @_;
	# $#array 获取最大下标
	foreach(0..$#array){
		if ($what eq $array[$_]){
			return $_;
		}
	}
	-1;
}

my @names = qw/ fred bar betty diao gosi /;
my $result = &find("gosi",@names);

print "find:$result\n";



sub running_sum(){
	state $sum = 0;
	state @numbers;

	# print "$sum\n";
	# print "@numbers\n";


	foreach my $number (@_) {
		# print "$number\n";
		push @numbers, $number;
		$sum += $number;
	}

	say "the sum of (@numbers) is $sum";
}


&running_sum(5,6);
&running_sum(1..3);
&running_sum(4);
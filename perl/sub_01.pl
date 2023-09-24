#!/usr/bin/perl

use 5.010;
# perl sub_01.pl

print "hello\n";


sub marine {
	$n += 1;
	print "num:$n\n";
}

&marine;
&marine;
&marine;
&marine;


sub calc {
	$a = $fred+$barney;
	$b = $fred-$barney;
	$c = $fred*$barney;
	$d = $fred/$barney;
}
$fred = 30;
$barney = 3;
$result = &calc;
print "$result\n";

sub larger {
	if($fred>$barney){
		$fred;
	} else{
		$barney;
	}
}

$fred = 30;
$barney = 3;
$result = &larger;
print "larger:$result\n";



sub max{
	if ($_[0]>$_[1]){
		$_[0];
	} else{
		$_[1];
	}
}


$result = &max(10,15);
print "max:$result\n";


sub max2{
	my($max_v) = shift @_;
	# print "max2:$max_v\n";
	foreach(@_){
		if ($_ > $max_v){
			$max_v = $_;
			# print "max2:$max_v\n";
		} else {
			$max_v;
		}
	}
	$max_v;
	# print "max2:$max_v\n";
}

$result = &max2(10,15,22,777,8888,11);
print "max2:$result\n";

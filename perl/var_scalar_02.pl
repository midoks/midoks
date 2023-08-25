#!/usr/bin/perl

# perl var_scalar_02.pl

$a = "hello\n\n\n";

$b = chomp($a);
print $a;

# $name = <STDIN>;
# print "你的输入值:".$name;

$count = 0;
while ($count<=10) {
	print $count."\n";
	$count += 2;
}

print "\n";
$x = 1;
print $x+1;

print "\n";

print "Hi:" .$ss;
print "\n";

undef $kk;
print $kk;
print "\n";


# $madona = <STDIN>;
$madona = <STDIN>;

if (defined($madona)){
	print ("googd\n");
} else{
	print ("bad\n");
}


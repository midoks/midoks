#!/usr/bin/perl
# perl grep_map_01.pl

my @arr  = ('cat', 'dog');
my $item = shift(@arr); # 'cat'
print "$item\n";

my @animals = ("cat");
push(@animals, "mouse"); # ("cat", "mouse")

my @colors = ("red");
push(@colors, ("blue", "green")); # ("red", "blue", "green")

print "@animals\n";
print "@colors\n";


sub big_money{
	my $number = sprintf "%.2f" , shift;
	# print "$number\n";

	1 while $number =~ s/^(-?\d+)(\d\d\d)/$1,$2/;

	$number =~ s/^(-?)/$1\$/;
	$number;
}

print (big_money 123123.123123);

my @data = (4.75, 1.5, 2, 1234, 6.9456, 12345678.9, 29.95);
my @formatted_data;


print "@data\n";

foreach (@data){
	push @formatted_data, &big_money($_);
}

print "@formatted_data\n";


my @formatted = map { &big_money($_) } @data;

print "------\n";
print "@formatted\n";

print "------\n";
print "the money number are:\n", map { sprintf "%25s\n", $_ } @formatted;

print "------\n";
print "the money number are:\n", map { sprintf "%25s\n", &big_money($_) } @data;

print "------\n";
print "some powers of two are:\n", map "\t" . (2**$_)."\n", 0..15;


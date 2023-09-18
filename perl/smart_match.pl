#!/usr/bin/perl

use 5.010;
# perl smart_match.pl

my $name = "fred find";

print  "aaa\n" if $name =~ /fred/;

print  "aaa\n" if $name ~~ /fred/;

my %names = ("one"=>1,"two"=>2,"there"=>3,"four"=>4,);

my $flag = 0;
foreach my $key ( keys %names) {
	next unless $key =~ /four/;
	$flag = $key;
	last;
}

print "i find 'four' .\n" if $flag;
print "i find 'four' .\n" if %names ~~ /four/;
#!/usr/bin/perl

use 5.010;

# perl file_02.pl


print "hello\n";


my @files = glob "*.pl";# *.pl
print "$files\n";
foreach (@files){
	print "$_\n";
}

print "\n\n";

my @files = <*.txt>;
foreach (@files){
	print $_."\n";
}

# unlink "t.txt";
# unlink glob "*.jj";

# rmname "t.xt", "n.txt";

my $now = time;
my $y = $now-24*60*60;
print "$now\n";


# utime $now, $y ,"n.txt";
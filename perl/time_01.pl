#!/usr/bin/perl

use 5.010;
# 时间起点
# 1970-1-1 00:00:00

# perl time_01.pl


print "hello\n";

print localtime();
print "\n";

print gmtime();
print "\n";

my $now = localtime();
print "$now\n";

my $now = gmtime();
print "$now\n";

my $time = localtime 3600;
print "$time\n";


# print "$now\n";
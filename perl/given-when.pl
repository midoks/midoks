#!/usr/bin/perl

# perl -e given-when.pl pp

use 5.010;
use strict;
use warnings;
# use feature qw( switch );
my $name = $ARGV[0];

# print "$name"
# eval {
# 	if ( $name =~ /fred/i ) { print "d1\n"}
# elsif ($name =~ /pp/i) {print "d2\n" }
# else{ print "d3\n" }
# }

# if ($@) {
# 	print "$@"
# }
# print "$name"

given($name){
	when ( /fred/i ) { print "1\n" }
	when ( /^pp/ ) { print "2\n" }
	default { print "ss2\n" }
}

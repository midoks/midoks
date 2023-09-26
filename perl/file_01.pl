#!/usr/bin/perl

use 5.010;

# perl file_01.pl


print "hello\n";


# mkdir "a";
# rmdir 'a';


my $dir = "./";

opendir DH, $dir or die "Cannot open $dir: $!";


foreach $file (readdir DH) {
	next if $file eq "." or $file eq "..";
	next if $file eq ".DS_Store";
	next if -d $file;
	next unless $file =~ /\.txt$/;
	print "$file\n";
}

closedir DH;

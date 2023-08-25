#!/usr/bin/perl

# perl ps.pl
# 进程管理
print("---------------\n");

system("date");
# exec("date");

print("---------------\n");


chomp(my $now = `date`);
print "$now\n";


open DATE, "date|" or die "cannot pipe form date:$!";

my $now = <DATE>;

print "open:".$now."\n";



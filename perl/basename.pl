#!/usr/bin/perl

# perl basenane.pl

use File::Basename;

use File::Spec;

# perl basename.pl

my $name = "/www/t/tt/a.php";

# method 1 
my $basename = basename $name;
my $dirname = dirname $name;

print "basename:$basename \n";
print "dirname:$dirname \n";


my $new_name = File::Spec->catfile($dirname , $basename);
print "new_name:$new_name \n";

#!/usr/bin/perl

use 5.010;

# perl file_op.pl

print "hello\n";

# while(<STDIN>){
# 	print "I saw $_";
# }


# open TXTFILE, "<", "given-when.pl";
# while (<TXTFILE>) {
# 	print $_;
# }
# close TXTFILE;

# open TXTFILE_TMP, ">", "t.txt";
# print TXTFILE_TMP "13";
# close TXTFILE_TMP;


# open TXTFILE_TMP, ">>", "t.txt";
# print TXTFILE_TMP "13\n";
# close TXTFILE_TMP;


if (! open TXTFILE_TMP, "<", "p.txt"){
	die "不能打开:$!\n";
}




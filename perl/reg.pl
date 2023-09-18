#!/usr/bin/perl

use 5.010;

# perl reg.pl

print "hello\n";

$_ = "banny and fred";

if (/fred/){ print "配上了!\n"; } else { print "没配上!\n"; }
if (m/fred/){ print "配上了!\n"; } else { print "没配上!\n"; }
if (m{fred}){ print "配上了!\n"; } else { print "没配上!\n"; }
if (m<fred>){ print "配上了!\n"; } else { print "没配上!\n"; }
if (m(fred)){ print "配上了!\n"; } else { print "没配上!\n"; }

$_ = "banny and fred http://www.baidu.com";
if (m%http://%){ print "配上了!\n"; } else { print "没配上!\n"; }


$some_other = "I dream of betty be";
if ($some_other =~ /\brub/){ print "配上了!\n"; } else { print "没配上!\n"; }

print "would you like to play a game? \n";
chomp($_ = <STDIN> );
if (m{yes}i){
	print "good body!\n";
} else{
	print "ok!\n";
}


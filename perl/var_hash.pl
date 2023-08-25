#!/usr/bin/perl

# perl var_hash.pl

print "hello\n";

$score["张飞"] = 99;
$score["刘备"] = 76;

print $score["刘备"]."\n";

foreach $person ( qw/张飞 刘备/){
	print $person.":".$score[$person]."\n";
}

%aa = ("ff"=>"2",1=>"ok");
print $aa{1}."\n";

@t = %aa;
print "@t\n";

@b = reverse %aa;
print "@b\n";

my @v = values %aa;
print "@v\n";

my @k = keys %aa;
print "@k\n";

while (($key, $value) = each %aa){
	print "each:$key => $value\n";
}


foreach $key (sort keys %aa) {
	print "foreach sort key:$key => $aa{$key}\n";
}


if (exists $aa{"b"}){
	print "hash hash b\n";
} else {
	print "hash not has key b\n";
}

delete $aa{"b"};
@tt = %ENV;
print "@tt\n";


print ".......---------------.......";
while (($key, $value) = each %ENV){
	print "env:$key => $value\n";
}


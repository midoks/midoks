#!/usr/bin/perl


print "hello world\n";

sub do_someting{
	# ...
}

foreach my $person (qw/ fr wi be ba /) {
	eval {
		# do something risky...
		print $person 
		open FILE, "<$person" or die "Can`t open file '$person':$!";
		my($total, $count);
		while (<FILE>) {
			$total += $_;
			$count++;
		}

		my $average = $total / $count;

		print "average for file $person was $average\n"

		&do_someting($person, $average);
	};

	if ($@) {
		print "An error occured ($@),\n"
	}
}

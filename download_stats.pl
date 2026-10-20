#!/usr/bin/env perl

use strict;
use warnings;

use JSON::PP;

my $json = JSON::PP->new->ascii->pretty->allow_nonref;
my $response = `curl -s "https://api.github.com/repos/agx-hv/1hpsi/releases"`;
my $content = $json->decode($response);

for my $line (@$content) {
    my $name = %$line{"name"};
    print "$name\t";
    my $assets = %$line{"assets"};
    my $asset = @$assets[0];
    my $count = %$asset{"download_count"};
    print "Downloads: $count\n";
}


#!/usr/bin/env perl

use strict;
use warnings;

use HTTP::Tiny;
use JSON::PP;

my $json = JSON::PP->new->ascii->pretty->allow_nonref;
my $response = HTTP::Tiny->new->get("https://api.github.com/repos/agx-hv/1hpsi/releases");
my $content;

if ($response->{success}) {
    $content = $json->decode($response->{content});
} else {
    warn "Failed: $response->{status} $response->{reason}\n";
}

for my $line (@$content) {
    my $name = %$line{"name"};
    print "$name\t";
    my $assets = %$line{"assets"};
    my $asset = @$assets[0];
    my $count = %$asset{"download_count"};
    print "Downloads: $count\n";
}


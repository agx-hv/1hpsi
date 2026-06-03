#!/usr/bin/env perl

use strict;
use warnings;
use Tie::File;
use Term::ANSIColor qw(:constants);

# Compiled regex patterns for semver
my %re = (
    "toml" => qr/^\s*version\s*=\s*"(\d+\.\d+\.\d+.*)"/,
    "json" => qr/^\s*\"version\"\s*:\s*"(\d+\.\d+\.\d+.*)"/,
);

# Files containing semver
my $cargo_toml = "src-tauri/Cargo.toml";
my $tauri_conf = "src-tauri/tauri.conf.json";

# Returns semver from provided file
sub get_file_version {
    my ($fname) = @_;
    my ($ext) = $fname =~ /.+\.(toml|json)$/;
    my $re = $re{$ext};
    my $i = 1;

    tie my @lines, "Tie::File", $fname or die "Cannot open $fname $!";
    for my $row (@lines) {
        if (my ($version) = $row =~ $re) {
            print CYAN "Found $version in $fname:", RESET, "\n";
            print "\t$i |\t$row\n\n";
            untie(@lines);
            return $version;
        }
        ++$i;
    }
    untie(@lines);
}

# Returns incremented patch version by one 
sub bump_version {
    my ($old_version) = @_;
    my ($major, $minor, $patch) = $old_version =~ /(\d+)\.(\d+)\.(\d+).*/;
    ++$patch;
    return "$major.$minor.$patch";
}

# Returns the newest (highest) semver from the provided args
sub get_newest_version {
    my @versions = @_;
    my @arr;
    for my $v (@versions) {
        my ($major, $minor, $patch) = $v =~ /(\d+)\.(\d+)\.(\d+).*/;
        my %h = (
            major => $major,
            minor => $minor,
            patch => $patch,
            name => $v,
        );
        push @arr, \%h;
    }
    @arr = sort {
        $a->{major} <=> $b->{major}
            or
        $a->{minor} <=> $b->{minor}
            or
        $a->{patch} <=> $b->{patch}
    } @arr;
    return $arr[-1]{name};
}

# Replace version number in file and output the diff to stdout
sub set_file_version {
    my ($fname, $newver) = @_;
    my ($ext) = $fname =~ /.+\.(toml|json)$/;
    my $re = $re{$ext};
    my $i = 1;

    tie my @lines, 'Tie::File', $fname or die "Cannot open $fname $!";
    print "\n---------- Modifying $fname ----------\n\n";
    for my $row (@lines) {
        if (my ($version) = $row =~ $re) {
            print RED "- $i |\t $row", RESET, "\n";
            $row =~ s/$version/$newver/;
            print GREEN "+ $i |\t $row", RESET, "\n";
        }
        ++$i;
    }
    untie(@lines);
}

# Main
print "Searching for versions...\n\n";

my $v1 = get_file_version($cargo_toml);
my $v2 = get_file_version($tauri_conf);

# Prompt user if versions are different in both files
if ($v1 ne $v2) {
    warn YELLOW "Version mismatch! Continue? (y/N)", RESET, "\n";
    my $stdin = <STDIN>;
    chomp($stdin);
    die "Aborted\n" if (lc($stdin) ne lc("y"));
}

# Get newest semver and bump by one in patch number
my $newver = bump_version(get_newest_version($v1, $v2));

# Prompt user to use default bump or provide their own
print "Provide the new version number (default: $newver): ";
my $stdin = <STDIN>;
chomp($stdin);

if (length $stdin == 0) {
    print "Using default $newver as new version...\n";
} else {
    $newver = $stdin;
    print "Using provided $newver as new version...\n";
}

# Edit both files and show diff
set_file_version($cargo_toml, $newver);
set_file_version($tauri_conf, $newver);

# End script
print "\n\nPress ENTER to continue ";
<STDIN>;

use strict;
use warnings;

package JSON::Tiny;

sub to_json {
    my $value = shift;

    my $ref = ref $value;
    if ( !$ref ) {
        if ( !defined( $value ) ) {
            return 'null'
        }
        elsif ( $value =~ m/^\d+(?:\.\d+)?$/ ) {
            return $value
        }
        else {
            my $copy = $value;
            $copy =~ s/\\/\\\\/g;
            $copy =~ s/"/\\"/g;

            return qq/"$copy"/
        }
    }
    elsif ( $ref eq 'HASH' ) {
        my @stack;
        foreach my $key ( keys %$value ) {
            my $value = to_json( $value->{$key} );
            push @stack, qq/"$key" : $value/;
        }
        return "{" . (join ', ', @stack) . "}"
    }
    elsif ( $ref eq 'ARRAY' ) {
        my @stack;
        foreach my $value ( @$value ) {
            push @stack, to_json( $value );
        }
        return "[" . (join ', ', @stack) . "]"
    }
}

1;

#!/usr/bin/env bash

perl -gne 'my $s=0;while(/mul\((\d{1,3}),(\d{1,3})\)/g){$s+=$1*$2};print "Part 1: ",$s,"\n"' < ${1:-example}
perl -gne 'my $s=0;s/\R//g;s/$/do/gm;s/don'\''t.*?do(?!n'\''t)/do/g;while(/mul\((\d{1,3}),(\d{1,3})\)/g){$s+=$1*$2};print "Part 2: ",$s,"\n"'< ${1:-example}

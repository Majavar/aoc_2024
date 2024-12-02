#!/usr/bin/env bash

jq -Rs '
    split("\n") |
    map(split(" ") | map(tonumber)) |
    map([., [null] + .] | transpose | map(select(.[0] and .[1])) | map(.[0] - .[1])) |
    map(select(((.|min) >= 1 and (.|max) <=3) or ((.|min) >= -3 and (.|max) <= -1))) |
    length
' ${1:-example}

jq -Rs '
    split("\n") |
    map(split(" ") | map(tonumber)) |
    map(
        . as $report |
        [range(. | length)] |
        map(. as $exclude | $report | to_entries | map(select(.key != $exclude)) | map(.value)) + [$report]
    ) |
    map(. |
        map([., [null] + .] | transpose | map(select(.[0] and .[1])) | map(.[0] - .[1])) |
        map(select(((.|min) >= 1 and (.|max) <=3) or ((.|min) >= -3 and (.|max) <= -1))) |
        length
    ) |
    map(select(. != 0)) |
    length
' ${1:-example}
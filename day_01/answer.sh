#! /usr/bin/env bash

filename="${1:-example}"
sum=0
left=()
right=()

while read -r -a values; do
    left+=("${values[0]}")
    right+=("${values[1]}")
done < $filename

IFS=$'\n' left_sorted=($(sort -h <<<"${left[*]}"))
IFS=$'\n' right_sorted=($(sort -h <<<"${right[*]}"))

for i in "${!left_sorted[@]}"; do
    (( sum += $(echo $(( ${left_sorted[i]} - ${right_sorted[i]} )) | sed 's/^-//' ) ))
done

echo "Part 1:" $sum

sum=0
for value in "${left[@]}"; do
    match=$(cat $filename | grep -c " ${value}$")
    (( sum += value * match ))
done

echo "Part 2:" $sum

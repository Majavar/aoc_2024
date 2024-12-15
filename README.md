# Advent of code 2024
[Advent of Code website](https://adventofcode.com)

## Day 01
Language: Bash

How to launch:
```sh
./day_01/{answer.sh,input}
```

Remarks:
* Not optimized at all. Expect around 10s to complete

## Day 02
Language: jq

How to launch:
```sh
./day_02/{answer.sh,input}
```

Remarks:
* Expect JQ to be in the path

## Day 03
Language: perl

How to launch:
```sh
./day_03/{answer.sh,input}
```

Remarks:
* One liner for each part !

## Day 04
Language: Scratch

How to launch:
* Load program
* Import `input` into list `input`
* Click on the green flag

## Day 05
Language: Prolog

How to launch:
```sh
swipl ./day_05/{answer.pl,input}
```

## Day 06
Language: sqlite

How to launch:
```
./day_06/{answer.sh,input}
```

Remarks:
* Takes around 4 minutes to complete.

## Day 06 part 2
Language Rust

Remark:
* Very basic. Could be greatly improved by adding obstacles only on position visited in part 1.

## Day 07
Language: Clojure

How to launch:
```sh

```

Remarks:
* Takes around 5 seconds to complete. Should replace || with 10^log(x)

## Day 08
Language: Python

How to launch:
```
./day_08/{answer.py,input}
```

## Day 09
Language: C

How to launch (from day_09 folder):
```sh
gcc main.c -o main && ./main < input
```

## Day 10
Language: Rust

How to launch (from day_10 folder):
```sh
cargo run < input
```

## Day 11
Language: C++

How to launch (from day_11 folder):
```sh
gcc main.cpp -o main && ./main < input
```

## Day 12
Language: Julia

How to launch:
```sh
julia ./day_07/{main.jl,input}
```

Remarks:
* First time in Julia !

## Day 13
Language: Ansible

How to launch:
```sh
ansible-playbook day_13/aoc.yml -e input_file=input
```

Remarks:
* Code does not consider case where a and b are multiples of each other

## Day 14
Language: Lua

How to launch:
```
lua aoc.lua input
```

Remarks:
* For part 2, find interesting pattern. Replace `s` width first occurrence of the pattern and `f` with frequency. Loop until christmas tree.
* Improvement idea for part 2: Search a big group of contiguous #

## Day 15
Language: Rust

How to build & launch
```
cargo run < input
```

#!/usr/bin/env bash

dirname=$(dirname $0)
sqlite3 ${dirname}/advent_of_code.db "create table input(line TEXT)" ".import --csv ${1-example} input" ".read ${dirname}/aoc.sql"
rm ${dirname}/advent_of_code.db
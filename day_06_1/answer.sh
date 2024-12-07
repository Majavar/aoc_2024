sqlite3 advent_of_code.db "create table input(line TEXT)" ".import --csv ${1-example} input" ".read aoc.sql"
rm advent_of_code.db
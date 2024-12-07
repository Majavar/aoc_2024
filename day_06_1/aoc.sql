create table size as select count() as height, max(length(line)) as width from input;

create table grid as
with col as (
    select 1 x
    union all
    select x + 1 from col where x < (select width from size)
)
select
    col.x - 1 x,
    i.y - 1 y,
    substring(i.line, col.x, 1) value
from
    (select row_number() over () y, line from input) i
    cross join col;

create table directions(name text, x integer, y integer, next text);
insert into directions values ('N', 0, -1, 'E'), ('E', 1, 0, 'S'), ('S', 0, 1, 'W'), ('W', -1, 0, 'N');

create table visited as 
with visited as (
    select x, y, 'N' direction, 0 step from grid where value = '^'
    union all
    select
        v.x + d.x x,
        v.y + d.y y,
        v.direction,
        v.step + 1 step
    from
        visited v
        join directions d on v.direction = d.name
        join grid g on g.x = v.x + d.x and g.y = v.y + d.y
    where
        coalesce(g.value, '#') != '#'
    union all
    select
        v.x x,
        v.y y,
        d.next direction,
        v.step + 1 step
    from
        visited v
        join directions d on v.direction = d.name
        join grid g on g.x = v.x + d.x and g.y = v.y + d.y
    where
        coalesce(g.value, '.') = '#'
)
select * from visited;

create table results as
select 'Part 1: ' part, count() from (select distinct x, y from visited)
union all
select 'Part 2: ' part, 0;

.headers off
.mode column
.output
select * from results;
.exit 0

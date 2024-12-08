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

-- Part 2

create table simulations as
select row_number() over () id, x, y from (select distinct x, y from visited except select x, y from grid where value = '^');

create table obstacles as
select
    s.id sid,
    g.x,
    g.y
from
    grid g
    cross join simulations s
where
    g.value = '#'
union all
select
    s.id sid,
    s.x,
    s.y
from
    simulations s;

create table next_steps as
select
    o.sid,
    o.x,
    o.y,
    'N' direction,
    t.x next_x,
    max(t.y) next_y,
    'E' next_direction
from
    obstacles o
    join obstacles t on
        t.sid = o.sid and
        t.x = o.x + 1 and
        t.y < o.y
group by
    o.sid, o.x, o.y
union all
select
    o.sid,
    o.x,
    o.y,
    'E' direction,
    min(t.x) next_x,
    t.y next_y,
    'S' next_direction
from
    obstacles o
    join obstacles t on
        t.sid = o.sid and
        t.x > o.x and
        t.y = o.y + 1
group by
    o.sid, o.x, o.y
union all
select
    o.sid,
    o.x,
    o.y,
    'S' direction,
    t.x next_x,
    min(t.y) next_y,
    'W' next_direction
from
    obstacles o
    join obstacles t on
        t.sid = o.sid and
        t.x = o.x - 1 and
        t.y > o.y
group by
    o.sid, o.x, o.y
union all
select
    o.sid,
    o.x,
    o.y,
    'W' direction,
    max(t.x) next_x,
    t.y next_y,
    'N' next_direction
from
    obstacles o
    join obstacles t on
        t.sid = o.sid and
        t.x < o.x and
        t.y = o.y - 1
group by
    o.sid, o.x, o.y
union all
select
    t.sid,
    g.x,
    g.y,
    'N' direction,
    t.x next_x,
    max(t.y) next_y,
    'E' next_direction
from
    grid g
    join obstacles t on
        t.x = g.x and
        t.y < g.y
where
    g.value = '^'
group by t.sid;


create table all_simulations as
with all_sims as (
    select s.id sid, g.x, g.y, 'N' direction from grid g cross join simulations s where g.value = '^'
    union
    select
        s.sid,
        n.next_x x,
        n.next_y y,
        n.next_direction direction
    from
        all_sims s
        join next_steps n on
            s.sid = n.sid and
            s.x = n.x and
            s.y = n.y and
            s.direction = n.direction
) select * from all_sims;

create table results as
select 'Part 1: ' part, count() from (select distinct x, y from visited)
union all
select 'Part 2: ' part, total.cnt - outside.cnt from
    (select count(*) cnt from simulations) total
    cross join (select count(*) cnt from (
        select sid, x, y, direction from all_simulations
        except
        select sid, x, y, direction from next_steps
    )) outside;

.headers off
.mode column
.output
select * from results;
.exit 0

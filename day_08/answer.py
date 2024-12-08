#!/usr/bin/env python3

from collections import defaultdict
from itertools import count
import sys

with open(sys.argv[1] if len(sys.argv) == 2 else 'example' ) as file:
    data = file.read().strip().split("\n")
(width, height) = (len(data[0]), len(data))

antennas = defaultdict(list)
for (value, position) in [(data[y][x], (x, y)) for y in range(height) for x in range(width) if data[y][x] != '.']:
    antennas[value].append(position)

def antinodes_freq(l, r, freq):
    for f in freq:
        x = l[0] - f * (r[0] - l[0])
        y = l[1] - f * (r[1] - l[1])
        if 0 <= x < width and 0 <= y < height:
            yield (x, y)
        else:
            return

antinodes = {
    k: [a for l in v for r in v if l != r for a in antinodes_freq(l, r, range(1, 2))]
    for k,v in antennas.items()
}

print("Part 1:", len({p for v in antinodes.values() for p in v}))

antinodes = {
    k: [a for l in v for r in v if l != r for a in antinodes_freq(l, r, count(0))]
    for k,v in antennas.items()
}

print("Part 2:", len({p for v in antinodes.values() for p in v}))
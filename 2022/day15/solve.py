#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

sensors = {}
beacons = set()

xs = []
ys = []

def dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1]- b[1])

for y, line in enumerate(lines):
    parts = line.split()
    x = int(parts[2][:-1].split('=')[1])
    y = int(parts[3][:-1].split('=')[1])
    xs.append(x)
    ys.append(y)

    x2 = int(parts[-2][:-1].split('=')[1])
    y2 = int(parts[-1].split('=')[1])
    xs.append(x2)
    ys.append(y2)

    sensors[(x, y)] = abs(x - x2) + abs(y - y2)
    beacons.add((x2, y2))

maxd = max(sensors.values()) + 1

rangex = (min(xs), max(xs))
rangey = (min(ys), max(ys))

print(rangex, rangey, maxd)
print(sensors)

Y = 10
Y = 2000000

total = 0
for x in range(rangey[0] - maxd, rangey[1] + maxd):
    p = (x, Y)
    if p in beacons:
        continue

    no = False
    for s, d in sensors.items():
        if dist(p, s) <= d:
            total += 1
            break

print(total)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(30000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

tunnels = {}
valves = {}
distances = {}

for y, line in enumerate(lines):
    parts = line.split()
    rate = int(parts[4][5:-1])
    neigh = [x.rstrip(',') for x in parts[9:]]
    tunnels[parts[1]] = neigh
    if rate > 0:
        valves[parts[1]] = rate

valveset = set(valves.keys())
print('Total', len(tunnels), 'has valve:', len(valveset))

paths = {}

def find_closes_paths(start):
    dist = {start: 0}
    worklist = [start]

    while worklist:
        first = worklist[0]
        worklist = worklist[1:]
        d = dist[first] + 1

        for n in tunnels[first]:
            if n in dist:
                assert dist[n] <= d
            else:
                dist[n] = d
                worklist.append(n)
    return dist

for v in tunnels.keys():
    paths[v] = find_closes_paths(v)

best = 0

def find(pos, time, total, enabled):
    global best
    if time <= 0:
        if total > best:
            print(total, enabled)
            best = total
    else:
        for c in list(valveset - enabled):
            distance = paths[pos][c]
            time2 = time - distance - 1
            total2 = total
            total2 += max(0, time2 * valves[c])

            enabled.add(c)
            find(c, time2, total2, enabled)
            enabled.remove(c)

find('AA', 30, 0, set())
print(best)
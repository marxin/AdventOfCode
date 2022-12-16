#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain, combinations
from functools import reduce

sys.setrecursionlimit(30000)

def powerset(iterable):
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))


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

def find(pos, time, total, enabled, maximum):
    if time <= 0 or len(enabled) == len(valveset):
        if total > maximum[0]:
            maximum[0] = total
    else:
        for c in valveset - enabled:
            distance = paths[pos][c]
            time2 = time - distance - 1
            total2 = total
            total2 += max(0, time2 * valves[c])

            enabled.add(c)
            find(c, time2, total2, enabled, maximum)
            enabled.remove(c)

best = [0]
find('AA', 30, 0, set(), best)
print(best[0])

splits = list(powerset(valveset))

totalbest = 0

for i, split in enumerate(splits):
    split = set(split)
    split2 = valveset - split
    
    best1 = [0]
    best2 = [0]
    find('AA', 26, 0, split, best1)
    find('AA', 26, 0, split2, best2)
    t = best1[0] + best2[0]
    if t > totalbest:
        totalbest = t
        print(i, '/', len(splits))
        print(list(split), list(split2))
        print('BEST', totalbest)

print(totalbest)
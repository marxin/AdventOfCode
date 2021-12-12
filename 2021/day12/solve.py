#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

d = {}

for line in lines:
    l, r = line.split('-')
    if l not in d:
        d[l] = []
    d[l].append(r)

    if r not in d:
        d[r] = []
    d[r].append(l)

total = 0

def find_path(point, seen, twice):
    global total

    if point == 'end':
        total += 1
    else:
        for neighbor in d[point]:
            if neighbor == 'start':
                continue
            elif neighbor.lower() == neighbor:
                if neighbor not in seen:
                    seen.add(neighbor)
                    find_path(neighbor, seen, twice)
                    seen.remove(neighbor)
                elif not twice:
                    find_path(neighbor, seen, True)
            else:
                find_path(neighbor, seen, twice)


print(d)
find_path('start', set(), False)
print(total)
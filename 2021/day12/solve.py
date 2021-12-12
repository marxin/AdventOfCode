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

def find_path(point, seen):
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
                    find_path(neighbor, seen)
                    seen.remove(neighbor)

            else:
                find_path(neighbor, seen)


print(d)
find_path('start', set())
print(total)
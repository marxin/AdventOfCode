#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

d = set()
folding = []

for line in lines:
    parts = line.split(',')
    if len(parts) == 2:
        d.add((int(parts[0]), int(parts[1])))
    else:
        parts = line.split(' ')[-1].split('=')
        if len(parts) == 2:
            folding.append(parts)

for direction, line in folding:
    line = int(line)

    if direction == 'y':
        d2 = set()
        for pos in d:
            if pos[1] > line:
                delta = pos[1] - line
                pos = (pos[0], line - delta)
            d2.add(pos)
    elif direction == 'x':
        d2 = set()
        for pos in d:
            if pos[0] > line:
                delta = pos[0] - line
                pos = (line - delta, pos[1])
            d2.add(pos)
    else:
        assert False
    d = d2

print(d)

for y in range(20):
    for x in range(60):
        print('#' if (x, y) in d else '.', end='')
    print()
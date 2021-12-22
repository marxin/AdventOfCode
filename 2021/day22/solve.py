#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = 50
d = set()

for i, line in enumerate(lines):
    print(line)
    op, coords = line.split()
    coords = [[int(y) for y in x[2:].split('..')] for x in coords.split(',')]
    for x in range(max(-N, coords[0][0]), min(N, coords[0][1]) + 1):
        for y in range(max(-N, coords[1][0]), min(N, coords[1][1]) + 1):
            for z in range(max(-N, coords[2][0]), min(coords[2][1], N) + 1):
                if op == 'on':
                    d.add((x, y, z))
                else:
                    d.discard((x, y, z))

print(len(d))
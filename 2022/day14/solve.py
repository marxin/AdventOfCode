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

START = (500, 0)

points = set()

for line in lines:
    parts = [[int(y) for y in x.split(',')] for x in line.split('->')]
    for i in range(len(parts) - 1):
        x1, y1 = parts[i]
        x2, y2 = parts[i + 1]        

        if x1 == x2:
            y = min(y1, y2)
            for y in range(min(y1, y2), max(y1, y2) + 1):
                points.add((x1, y))
        else:
            assert y1 == y2
            for x in range(min(x1, x2), max(x1, x2) + 1):
                points.add((x, y1))

maxy = max([p[1] for p in points]) + 1
start_points = len(points)

while True:
    p = START
    while True:
        if p[1] == maxy:
            print('done', len(points) - start_points)
            sys.exit(0)

        candidates = ((p[0], p[1] + 1), (p[0] - 1, p[1] + 1), (p[0] + 1, p[1] + 1))
        moved = False
        for cand in candidates:
            if cand not in points:
                p = cand
                moved = True
                break

        if not moved:
            # we are done with this part
            points.add(p)
            break
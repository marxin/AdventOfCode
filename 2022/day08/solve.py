#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = len(lines)

good = 0

def check_point(c, x, y, lines):
    for move in MOVES:
        for i in range(1, N + 1):
            xx, yy = x + i * move[0], y + i * move[1]
            if xx < 0 or xx >= N or yy < 0 or yy >= N:
                return True
            if int(lines[yy][xx]) >= c:
                break
    return False

def get_seen_trees(c, x, y, lines):
    values = []
    for move in MOVES:
        maximum = 0
        for i in range(1, N + 1):
            xx, yy = x + i * move[0], y + i * move[1]
            if xx < 0 or xx >= N or yy < 0 or yy >= N:
                values.append(maximum)
                break
            maximum += 1
            neighbor = int(lines[yy][xx])
            if neighbor >= c:
                values.append(maximum)
                break
    return reduce(lambda x, y: x * y, values)


for y, line in enumerate(lines):
    for x, c in enumerate(line):
        h = int(c)
        if check_point(h, x, y, lines):
            good += 1

print(good)

best_view = 0

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        h = int(c)
        v = get_seen_trees(h, x, y, lines)
        best_view = max(best_view, v)

print(best_view)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from math import prod

MOVES = ((1, 0), (0, 1), (-1, 0), (0, -1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        data[(x, y)] = int(c)

starts = []

for key, value in data.items():
    done = True
    for move in MOVES:
        pos = (key[0] + move[0], key[1] + move[1])
        if pos in data:
            if data[pos] <= value:
                done = False
    if done:
        starts.append(key)

def get_bigger(key):
    for move in MOVES:
        pos = (key[0] + move[0], key[1] + move[1])
        if pos in data and data[pos] > data[key] and data[pos] < 9:
            yield pos

def size_of_start(start):
    seen = set([start])
    stack = [start]

    while stack:
        p = stack.pop()
        for bigger in get_bigger(p):
            seen.add(bigger)
            stack.append(bigger)

    return len(seen)

sizes = sorted([size_of_start(x) for x in starts], reverse=True)
print(prod(sizes[:3]))
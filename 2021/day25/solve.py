#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter

MOVES = ((1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c != '.':
            data[(x, y)] = c

def printme(data):
    for y in range(H):
        for x in range(W):
            v = data.get((x, y), '.')
            print(v, end='')
        print()
    print()

def move(data):
    moved = 0

    for i, c in enumerate(('>', 'v')):
        data2 = {}
        for pos, value in data.items():
            if value != c:
                data2[pos] = value
                continue
            move = MOVES[i]
            pos2 = ((pos[0] + move[0]) % W, (pos[1] + move[1]) % H)
            if pos2 not in data:
                moved += 1
                data2[pos2] = c
            else:
                data2[pos] = c
        data = data2
    return (moved, data)

i = 0
while True:
    moved, data = move(data)
    i += 1
    print(i, moved)
    # printme(data)
    if moved == 0:
        break

print(i)
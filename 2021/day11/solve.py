#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
times = 0

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        data[(x, y)] = int(c)

def flash(pos, flashed):
    if pos in flashed:
        return

    if data[pos] > 9:
        flashed.add(pos)
        for i in range(-1, 2):
            for j in range(-1, 2):
                if i != 0 or j != 0:
                    pos2 = (pos[0] + i, pos[1] + j)
                    if pos2 in data and pos2 not in flashed:
                        data[pos2] += 1
                        flash(pos2, flashed)

Y = len(lines)
X = len(lines[0])

def print_matrix():
    for y in range(Y):
        for x in range(X):
            print(data[(x, y)], end='')
        print()
    print()

print_matrix()

steps = 0
while True:
    steps += 1
    for y in range(Y):
        for x in range(X):
            data[(x, y)] += 1

    flashed = set()
    for y in range(Y):
        for x in range(X):
            flash((x, y), flashed)

    
    for f in flashed:
        data[f] = 0

    if len(flashed) == len(data):
        print(steps)
        break
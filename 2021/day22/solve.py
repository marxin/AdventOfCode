#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __getitem__(self, index):
        if index == 0:
            return self.x
        elif index == 1:
            return self.y
        elif index == 2:
            return self.z
        else:
            assert False

    def __repr__(self):
        return str((self.x, self.y, self.z))

class Box:
    def __init__(self, data):
        self.start = Point(*[x[0] for x in data])
        self.end = Point(*[x[1] for x in data])
        assert self.start.x < self.end.x
        assert self.start.y < self.end.y
        assert self.start.z < self.end.z

    def __repr__(self):
        return f'Box({self.start}-{self.end})'


folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = 50
d = set()

minimum = [sys.maxsize, sys.maxsize, sys.maxsize]
maximum = [-sys.maxsize, -sys.maxsize, -sys.maxsize]

actions = []

for i, line in enumerate(lines):
    op, coords = line.split()
    coords = [[int(y) for y in x[2:].split('..')] for x in coords.split(',')]
    actions.append((op, Box(coords)))

for op, box in actions:
    for i in range(3):
        minimum[i] = min(box.start[i], minimum[i])
        maximum[i] = max(box.end[i], maximum[i])

    for x in range(max(-N, box.start.x), min(N, box.end.x) + 1):
        for y in range(max(-N, box.start.y), min(N, box.end.y) + 1):
            for z in range(max(-N, box.start.z), min(box.end.z, N) + 1):
                if op == 'on':
                    d.add((x, y, z))
                else:
                    d.discard((x, y, z))

print(len(d))

print(minimum)
print(maximum)
print(len(actions))
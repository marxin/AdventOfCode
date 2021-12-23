#!/usr/bin/env python3

import math
import os
import sys

from collections import defaultdict, Counter

DIM = 3

class Point:
    def __init__(self, coords):
        self.coords = tuple(coords)

    def __getitem__(self, index):
        return self.coords[index]

    def __repr__(self):
        return str(self.coords)

class Box:
    def __init__(self, start, end, on=None):
        self.start = start
        self.end = end       
        self.end_minus_one = Point([x -  1 for x in end.coords]) 
        self.on = on

        for i in range(len(self.start.coords)):
            assert self.start[i] < self.end[i]

    def volume(self):
        p = 1
        for i in range(3):
            p *= self.end[i] - self.start[i]
        return p

    def is_point_in(self, other, point):
        for i in range(DIM):
            if other.start[i] <= point[i] < other.end[i]:
                pass
            else:
                return False
        return True

    def isin(self, other):
        return self.is_point_in(other, self.start) and self.is_point_in(other, self.end_minus_one)

    def __repr__(self):
        return f'Box({self.start}:{self.end})'


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
    start = Point([x[0] for x in coords])
    end = Point([x[1] + 1 for x in coords])
    actions.append(Box(start, end, op == 'on'))

for box in actions:
    for i in range(3):
        minimum[i] = min(box.start[i], minimum[i])
        maximum[i] = max(box.end[i] - 1, maximum[i])

    for x in range(max(-N, box.start[0]), min(N, box.end[0]) + 1):
        for y in range(max(-N, box.start[1]), min(N, box.end[1]) + 1):
            for z in range(max(-N, box.start[2]), min(box.end[2], N) + 1):
                if box.on:
                    d.add((x, y, z))
                else:
                    d.discard((x, y, z))

actions = list(reversed(actions))
print(actions)

print(len(d))

print('Minimum:', minimum)
print('Maximum:', maximum)
print('Actions:', len(actions))

total = (maximum[0] - minimum[0] + 1) * (maximum[1] - minimum[1] + 1) * (maximum[2] - minimum[2] + 1)

# print(expected / total)
# print(math.log10(total))

a = Box(Point((0, 0, 0,)), Point((2, 3, 4,)))
b = Box(Point((1, 2, 0,)), Point((2, 3, 4,)))
assert a.volume() == 24
assert b.isin(a)
assert not a.isin(b)

sets = []
for _ in range(DIM):
    sets.append(set())

for i in range(DIM):
    for a in actions:
        sets[i].add(a.start[i])
        sets[i].add(a.end[i])

for i in range(DIM):
    sets[i] = sorted(list(sets[i]))
    print(len(sets[i]))

allcubes = []
for x in range(len(sets[0]) - 1):
    print(x)
    for y in range(len(sets[1]) - 1):
        for z in range(len(sets[2]) - 1):
            start = ((sets[0][x], sets[1][y], sets[2][z]))
            end = ((sets[0][x + 1], sets[1][y + 1], sets[2][z + 1]))
            # cube = Box(start, end)
            # allcubes.append(cube)

alltotal = sum([c.volume() for c in allcubes])
print('All total', alltotal, 'Total:', total)
assert alltotal == total
print('Volume is equal!')

expected = 2758514936282235

calculated = 0
for i, cube in enumerate(allcubes):
    if i % 100000 == 0:
        print(f'{i}/{len(allcubes)}: {calculated}')
    for a in actions:
        if cube.isin(a):
            if a.on:
                calculated += cube.volume()
            break

print(calculated)
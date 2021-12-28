#!/usr/bin/env python3

import math
import os
import sys

from collections import defaultdict, Counter

DIM = 3

class Box:
    def __init__(self, points, on=None):
        assert len(points) == DIM
        self.points = points
        self.on = on
        self.start = self._start()
        self.end = self._end()
        self.end_minus_one = self._end_minus_one()
        self.corners = list(self._get_corners())

        for d in range(DIM):
            assert len(points[d]) >= 2
            assert self.points[d] == sorted(self.points[d])
            assert len(set(self.points[d])) == len(self.points[d])

    def volume(self):
        p = 1
        for d in range(DIM):
            p *= self.points[d][-1] - self.points[d][0]
        return p
    
    def subcube_count(self):
        return math.prod(len(p) - 1 for p in self.points)

    def is_point_dimensions(self, other, point):
        total = 0
        for d in range(DIM):
            if other.points[d][0] <= point[d] < other.points[d][-1]:
                total += 1
        return total

    @staticmethod
    def is_point_in(other, point):
        for d in range(DIM):
            if other.points[d][0] <= point[d] < other.points[d][-1]:
                pass
            else:
                return False
        return True

    def _start(self):
        return tuple([x[0] for x in self.points])

    def _end(self):
        return tuple([x[-1] for x in self.points])

    def _end_minus_one(self):
        return tuple([p - 1 for p in self.end])

    def points_in(self, other):
        inter = 0
        if self.is_point_in(other, self.start):
            inter += 1
        if self.is_point_in(other, self.end_minus_one):
            inter += 1
        return inter

    def has_corner_in(self, other):
        for corner in self.corners:
            if self.is_point_in(other, corner):
                return True
        return False

    def split8(self):
        result = []
        if all(len(val) <= 2 for val in self.points):
            return []

        split = [len(x) // 2 for x in self.points]
        for x in (self.points[0][:split[0] + 1], self.points[0][split[0]:]):
            for y in (self.points[1][:split[1] + 1], self.points[1][split[1]:]):
                for z in (self.points[2][:split[2] + 1], self.points[2][split[2]:]):
                    if len(x) >= 2 and len(y) >= 2 and len(z) >= 2:
                        b = Box((x, y, z))
                        result.append(b)
        assert len(result) <= 8
        # if result:
        #    assert sum(x.volume() for x in result) == self.volume()
        return result

    def _get_corners(self):
        points = [self.start, self.end_minus_one]

        for x in (0, 1):
            for y in (0, 1):
                for z in (0, 1):
                    yield (points[x][0], points[y][1], points[z][2])

    def __repr__(self):
        return f'Box[{self.start()}:{self.end()})'

a = Box(([0, 4, 5], [0, 5, 6], [0, 7, 8]))
b = Box(([1, 3], [2, 3], [3, 4]))

assert a.points_in(b) == 0
assert b.points_in(a) == 2
s0 = list(Box(([0, 1], [0, 1], [0, 1])).split8())
assert not s0

a8 = list(a.split8())
assert len(a8) == 8
assert sum(c.volume() for c in a8) == a.volume()

c = Box(([0, 1, 9], [0, 1, 9], [0, 1, 9]))
c8 = list(c.split8())
assert len(c8) == 8

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = 50
data = set()

minimum = [sys.maxsize, sys.maxsize, sys.maxsize]
maximum = [-sys.maxsize, -sys.maxsize, -sys.maxsize]

actions = []

for i, line in enumerate(lines):
    op, coords = line.split()
    coords = [[int(y) for y in x[2:].split('..')] for x in coords.split(',')]
    for d in range(DIM):
        coords[d][-1] += 1
    actions.append(Box(coords, op == 'on'))

for box in actions:
    for d in range(DIM):
        minimum[d] = min(box.points[d][0], minimum[d])
        maximum[d] = max(box.points[d][-1] - 1, maximum[d])

"""
for box in actions:
    for d in range(DIM):
        minimum[d] = min(box.points[d][0], minimum[d])
        maximum[d] = max(box.points[d][-1] - 1, maximum[d])

    for x in range(max(-N, box.points[0][0]), min(N, box.points[0][-1]) ):
        for y in range(max(-N, box.points[1][0]), min(N, box.points[1][-1])):
            for z in range(max(-N, box.points[2][0]), min(N, box.points[2][-1])):
                if box.on:
                    data.add((x, y, z))
                else:
                    data.discard((x, y, z))
"""

actions = list(reversed(actions))

calculated1 = len(data)
print('Total:', calculated1)

print('Minimum:', minimum)
print('Maximum:', maximum)
print('Actions:', len(actions))

total = (maximum[0] - minimum[0] + 1) * (maximum[1] - minimum[1] + 1) * (maximum[2] - minimum[2] + 1)

sets = []
for _ in range(DIM):
    sets.append(set())

for d in range(DIM):
    for a in actions:
        sets[d] |= set(a.points[d])

for d in range(DIM):
    sets[d] = sorted(list(sets[d]))

megacube = Box(sets)
assert megacube.volume() == total
total_volume = megacube.volume()

calculated = 0
counter = 0
worklist = [megacube]
discovered = 0

while worklist:
    counter += 1
    if counter % 10000 == 0:
        print(counter, calculated, 'done:', f'{100.0 * discovered / total_volume:.3f} %', len(worklist), f'{100.0 * cube.volume() / total_volume:.4f} %')
    cube = worklist.pop()
    add_me = True
    for a in actions:
        inter = cube.points_in(a)
        if inter == 2:
            if a.on:
                calculated += cube.volume()
            break
        elif inter == 1 or a.points_in(cube) != 0 or cube.has_corner_in(a) or a.has_corner_in(cube):
            worklist += cube.split8()
            add_me = False
            break
    if add_me:
        discovered += cube.volume()
    

print(calculated)

# for d in data:
#     ff = False
#     for f in found:
#         if f.is_point_in(f, d):
#             ff = True
#             break
#     if not ff:
#         print('missing', d)
#         assert False

#print(calculated, calculated1)
#assert calculated == calculated1
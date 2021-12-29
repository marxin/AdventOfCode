#!/usr/bin/env python3

import math
import os
import sys

from collections import defaultdict, Counter

DIM = 3

class Box:
    def __init__(self, start, end, on=None):
        self.start = start
        self.end = end
        for d in range(DIM):
            assert self.start[d] < self.end[d]
        self.end_minus_one = self._end_minus_one()

        self.on = on
        self.enabled = False
        self.corners = list(self._get_corners())

    def volume(self):
        p = 1
        for d in range(DIM):
            p *= self.end[d] - self.start[d]
        return p
    
    @staticmethod
    def is_point_in(other, point):
        for d in range(DIM):
            if other.start[d] <= point[d] < other.end[d]:
                pass
            else:
                return False
        return True

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

    def _get_corners(self):
        points = [self.start, self.end_minus_one]

        for x in (0, 1):
            for y in (0, 1):
                for z in (0, 1):
                    yield (points[x][0], points[y][1], points[z][2])

    def __repr__(self):
        return f'Box[{self.start}:{self.end})'

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

b = Box([0, 0, 0], [1, 2, 3])
assert b.volume() == 1 * 2 * 3

N = 50
data = set()
actions = []
enabled = []

for i, line in enumerate(lines):
    op, coords = line.split()
    coords = [[int(y) for y in x[2:].split('..')] for x in coords.split(',')]
    start = [x[0] for x in coords]
    end = [x[1] + 1 for x in coords]
    actions.append(Box(start, end, op == 'on'))

print('Actions:', len(actions))

def get_cubes(enabled, action):
    result = []
    tosplit = []

    for ena in enabled:
        if ena.has_corner_in(action) and ena.points_in(action) != 2:
            tosplit.append(ena)
        else:
            assert ena.enabled
            result.append(ena)

    points = []
    for c in tosplit + [action]:
        points += [c.start, c.end]

    x_values = sorted(set(p[0] for p in points))
    y_values = sorted(set(p[1] for p in points))
    z_values = sorted(set(p[2] for p in points))


    for x in range(len(x_values) - 1):
        for y in range(len(y_values) - 1):
            for z in range(len(z_values) - 1):
                split = Box((x_values[x], y_values[y], z_values[z]), (x_values[x + 1], y_values[y + 1], z_values[z + 1]))
                for ena in tosplit:
                    result.append(split)
                    if split.points_in(ena) == 2:
                        split.enabled = True
                        break
    return result

for i, action in enumerate(actions):
    print(action)
    print(i + 1, '/', len(actions), len(enabled))

    if enabled:
        enabled2 = []
        splits = get_cubes(enabled, action)
        for split in splits:
            if action.on:
                if split.enabled or split.points_in(action) == 2:
                    split.enabled = True
                    enabled2.append(split)
            else:
                if split.enabled and not split.has_corner_in(action):
                    split.enabled = True
                    enabled2.append(split)
        enabled = enabled2
    elif action.on:
        enabled.append(action)

print(sum())
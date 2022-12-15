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

data = {}
H = len(lines)
W = len(lines[0])

sensors = {}
beacons = set()

xs = []
ys = []

def dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1]- b[1])

for y, line in enumerate(lines):
    parts = line.split()
    x = int(parts[2][:-1].split('=')[1])
    y = int(parts[3][:-1].split('=')[1])
    xs.append(x)
    ys.append(y)

    x2 = int(parts[-2][:-1].split('=')[1])
    y2 = int(parts[-1].split('=')[1])
    xs.append(x2)
    ys.append(y2)

    sensors[(x, y)] = abs(x - x2) + abs(y - y2)
    beacons.add((x2, y2))

print(sensors.values())
maxd = max(sensors.values()) + 1
print(sum(sensors.values()))

rangex = (min(xs), max(xs))
rangey = (min(ys), max(ys))

print(rangex, rangey, maxd)
print(sensors)

def in_sensor(p):
    for s, d in sensors.items():
        if dist(p, s) <= d:
            return True
    return False

MAXIMUM = 4000000

print('== part 2 ===')
print('sensors', len(sensors))

for i, (s, d) in enumerate(sensors.items()):
    print(i, '/', len(sensors), s, d)
    d += 1

    for x in range(d):
        p = (s[0] + x, s[1] + d - x)
        p2 = (s[0] + x, s[1] - (d - x))

        p3 = (s[0] - x, s[1] + d - x)
        p4 = (s[0] - x, s[1] - (d - x))

        points = (p, p2, p3, p4)
        for point in points:
            if point[0] < 0 or point[0] > MAXIMUM:
                continue
            if point[1] < 0 or point[1] > MAXIMUM:
                continue

            if not in_sensor(point):
                print('HOORAY')
                print(point)
                print(point[0] * MAXIMUM + point[1])
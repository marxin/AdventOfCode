#!/usr/bin/env python3

import os

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

points = {}

for line in lines:
    left, right = line.split(' -> ')
    x1, y1 = [int(x) for x in left.split(',')]
    x2, y2 = [int(x) for x in right.split(',')]
    
    if x1 == x2:
        if y2 < y1:
            y1, y2 = y2, y1
        for y in range(y1, y2 + 1):
            p = (x1, y)
            if p in points:
                points[p] += 1
            else:
                points[p] = 1

    if y1 == y2:
        if x2 < x1:
            x1, x2 = x2, x1
        for x in range(x1, x2 + 1):
            p = (x, y1)
            if p in points:
                points[p] += 1
            else:
                points[p] = 1

print(len([p for p in points.values() if p > 1]))
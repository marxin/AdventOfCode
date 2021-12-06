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

    diffX = x2 - x1
    diffY = y2 - y1
    diffXabs = abs(diffX)
    diffYabs = abs(diffY)

    if diffXabs == diffYabs or diffX == 0 or diffY == 0:
        stepX = diffX / diffXabs if diffX else 0
        stepY = diffY / diffYabs if diffY else 0

    for i in range(max(diffXabs, diffYabs) + 1):
        p = (x1 + i * stepX, y1 + i * stepY)
        if p in points:
            points[p] += 1
        else:
            points[p] = 1

print(len([p for p in points.values() if p > 1]))
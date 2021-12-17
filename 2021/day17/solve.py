#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from typing import Sequence

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
x, y = data.splitlines()[0].split(':')[-1].split(', ')
x = x[3:].split('..')
y = y[2:].split('..')

tstart = (int(x[0]), int(y[0]))
tend = (int(x[1]), int(y[1]))
print(tstart, tend)

def step(velocity):
    x = velocity[0]
    if x != 0:
        x = x - 1 if x > 0 else x + 1
    y = velocity[1]
    y -= 1
    return (x, y)

def isin(pos):
    return tstart[0] <= pos[0] <= tend[0] and tstart[1] <= pos[1] <= tend[1]

def isout(pos):
    minimum = min(tstart[1], tend[1])
    return pos[0] > tend[0] or pos[1] < minimum

def shoot(velocity):
    position = (0, 0)
    maxy = 0

    while True:
        maxy = max(maxy, position[1])
        amin = isin(position)
        # print(position, velocity, amin)
        if amin:
            return (True, maxy)
        if isout(position):
            return (False, maxy)

        position = (position[0] + velocity[0], position[1] + velocity[1])
        velocity = step(velocity)


counter = 0
RANGE = 300

for x in range(RANGE):
    for y in range(-RANGE, RANGE):
        v = (x, y)
        s = shoot(v)
        if s[0]:
            counter += 1

print(counter)
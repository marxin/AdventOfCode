#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0, 0), (0, -1, 0), (-1, 0, 0), (0, 1, 0), (0, 0, 1), (0, 0, -1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

data = {}

cubes = set()

maximum = 0

for y, line in enumerate(lines):
    p = tuple([int(x) for x in line.split(',')])
    cubes.add(p)
    m = max(p)
    if m > maximum:
        maximum = m
    assert min(p) >= 0
    

total = 0

for c in cubes:
    for m in MOVES:
        c2 = (m[0] + c[0], m[1] + c[1], m[2] + c[2])
        if c2 not in cubes:
            total += 1

print(total)


def flood(start):
    assert start not in cubes
    worklist = [start]
    water = {start}

    while worklist:
        c = worklist[0]
        worklist = worklist[1:]
        for m in MOVES:
            c2 = (m[0] + c[0], m[1] + c[1], m[2] + c[2])
            if max(c2) > maximum + 2:
                break
            elif min(c2) < -1:
                break

            if c2 not in cubes and c2 not in water:
                water.add(c2)
                worklist.append(c2)

    return water


# Part 2
print('Maximum in any axis', maximum)

water = flood((maximum + 1, maximum + 1, maximum + 1))
print('Water has', len(water))

total = 0

for c in cubes:
    for m in MOVES:
        c2 = (m[0] + c[0], m[1] + c[1], m[2] + c[2])
        if c2 in water:
            total += 1

print(total)
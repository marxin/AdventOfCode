#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

def contains(inter, inter2):
    return inter[0] <= inter2[0] and inter2[1] <= inter[1]

def contains_part(inter, inter2):
    return (inter[0] <= inter2[0] and inter2[0] <= inter[1]) or (inter[0] <= inter2[1] and inter2[1] <= inter[1])

tuples = []
for y, line in enumerate(lines):
    left, right = line.split(',')

    p = left.split('-')
    left = (int(p[0]), int(p[1]))
    p = right.split('-')
    right = (int(p[0]), int(p[1]))
    tuples.append((left, right))

total = 0
for left, right in tuples:
    if contains(left, right) or contains(right, left):
        total += 1
print(total)

total2 = 0
for left, right in tuples:
    if contains_part(left, right) or contains_part(right, left):
        total2 += 1

print(total2)

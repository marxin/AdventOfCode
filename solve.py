#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

FILENAME = 'sample.txt' if True else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        data[(x, y)] = c

print(lines)
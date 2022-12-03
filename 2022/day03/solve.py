#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations, accumulate
from collections import defaultdict, Counter
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

def get_rank(c):
    index = ord(c)
    return index - ord('a') + 1 if c.islower() else index - ord('A') + 27

suma = 0
for y, line in enumerate(lines):
    half = len(line) // 2
    first, second = set(line[:half]), set(line[half:])
    c = first & second
    assert len(c) == 1
    c = list(c)[0]
    suma += get_rank(c)

print(suma)

suma2 = 0
assert len(lines) % 3 == 0
for i in range(len(lines) // 3):
    sets = [set(lines[3 * i]), set(lines[3 * i + 1]), set(lines[3 * i + 2])]
    r = reduce(lambda x, y: x & y, sets)
    vals = list(r)
    assert len(vals) == 1
    suma2 += get_rank(vals[0])

print(suma2)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

ip = 0
increments = defaultdict(int)

for y, line in enumerate(lines):
    parts = line.split()
    match parts[0]:
        case 'noop':
            ip += 1
        case 'addx':
            inc = int(parts[1])
            increments[ip + 2] = inc
            ip += 2

def get_n(increments, n):
    x = 1
    for i in range(n):
        x += increments[i]
    return x * n

total = 0
for i in range(20, 220 + 1, 40):
    total += get_n(increments, i)
print(total)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

data = {}
H = len(lines)
W = len(lines[0])

start = None
end = None

flood = {}

for y, line in enumerate(lines):
    for x, c in enumerate(line):        
        data[(x, y)] = c
        match c:
            case 'S':
                start = (x, y)
            case 'E':
                end = (x, y)

flood[start] = 0
worklist = deque([start])

while worklist:
    first = worklist.popleft()
    steps = flood[first] + 1
    c1 = ord(data[first])

    for m in MOVES:
        pos = (first[0] + m[0], first[1] + m[1])
        if pos in data:
            c2 = data[pos]
            if c2 == 'E':
                c2 = 'z'
            if c1 == ord('S') or ord(c2) <= c1 + 1:
                if pos not in flood or steps < flood[pos]:
                    flood[pos] = steps
                    worklist.append(pos)

print(flood[end])
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))
DIRECTIONS = ('R', 'D', 'L', 'U')

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = 9
parts = (N  + 1) * [(0, 0)]
print(parts)

def get_move(diff):
    if abs(diff[0]) <= 1 and abs(diff[1]) <= 1:
        return (0, 0)
    return (diff[0] // abs(diff[0]) if diff[0] else 0, diff[1] // abs(diff[1]) if diff[1] else 0)

visited = set()
for y, line in enumerate(lines):
    direction, steps = line.split()
    visited.add(parts[-1])
    
    for i in range(int(steps)):
        move = MOVES[DIRECTIONS.index(direction)]
        H = parts[0]
        H = (H[0] + move[0], H[1] + move[1])
        parts[0] = H

        for j in range(1, N + 1):
            H = parts[j - 1]
            T = parts[j]

            diff = (H[0] - T[0], H[1] - T[1])
            move2 = get_move(diff)
            T = (T[0] + move2[0], T[1] + move2[1])
            parts[j] = T

        visited.add(parts[-1])

print(len(visited))
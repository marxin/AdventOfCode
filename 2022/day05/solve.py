#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

N = 9

first, second = data.split('\n\n')

columns = first.splitlines()[:-1]

piles = [''] * N
for i in range(N):
    for column in columns:
        piles[i] += column[4 * i + 1].strip()

for command in second.splitlines():
    parts = command.split()
    n, src, to = int(parts[1]), int(parts[3]) - 1, int(parts[5]) - 1
    move = piles[src][:n]
    piles[src] = piles[src][n:]
    # part 1
    # piles[to] = move[::-1] + piles[to]
    piles[to] = move + piles[to]

print(piles)

result = ''.join([p[0] for p in piles])
print(result)
#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

totals = []

for part in data.split('\n\n'):
    s = sum([int(x) for x in part.splitlines()])
    totals.append(s)

totals = sorted(totals, reverse=True)
print(totals[0])

print(sum(totals[:3]))
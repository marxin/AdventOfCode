#!/usr/bin/env python3

import os
import sys
import random

from itertools import product
from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

N = 14

assert len(lines) % N == 0
part = len(lines) // N
types = [[0, None] for _ in range(N)]

for i in range(N):
    chunk = lines[part * i:part * (i + 1)]
    assert len(chunk) == part
    if chunk[4] == 'div z 26':
        types[i][0] = 2
        types[i][1] = int(chunk[5].split()[-1])
    else:
        types[i][0] = 1
        types[i][1] = int(chunk[-3].split()[-1])

def solve(values):
    result = []
    z = 0
    for i in range(N):
        if types[i][0] == 1:
            w = values[0]
            values = values[1:]
            result.append(w)
            z = 26 * z + w + types[i][1]
        elif types[i][0] == 2:
            w = z % 26 + types[i][1]
            if 1 <= w <= 9:
                result.append(w)
                z //= 26
            else:
                return None
        else:
            raise RuntimeError
    result = ''.join(str(x) for x in result)
    return int(result)


max = 0
for values in product(range(9, 0, -1), repeat=N // 2):
    result = solve(values)
    if result and result > max:
        max = result
        print(max)
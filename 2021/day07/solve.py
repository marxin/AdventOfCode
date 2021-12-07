#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

values = [int(x) for x in lines[0].split(',')]

best = sys.maxsize

def fuel(distance):
    return (1 + distance) * distance // 2

for position in range(max(values) + 1):
    suma = sum(fuel(abs(x - position)) for x in values)
    if suma < best:
        best = suma

print(best)
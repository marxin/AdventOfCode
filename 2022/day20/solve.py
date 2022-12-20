#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

numbers = []

for y, line in enumerate(lines):
    n = int(line)
    numbers.append(n)

LEN = len(numbers)
L2 = LEN - 1

for i, n in enumerate(numbers.copy()):
    x = n % LEN
    assert x < LEN
    i = numbers.index(n)
    i2 = (i + n) % L2

    numbers = numbers[:i] + numbers[i + 1:]
    numbers = numbers[:i2] + [n] + numbers[i2:]
    print(i, n)
    # print(numbers)

index = numbers.index(0)
print()
print('zero index', index)

total = 0
for i in (1000, 2000, 3000):
    i2 = (index + i) % LEN
    print(i2, numbers[i2])
    total += numbers[i2]

print('result', total)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()

KEY = 811589153
numbers = [int(x) * KEY for x in lines]
numbers = deque(enumerate(numbers))
LEN = len(numbers)

for k in range(10):
    for i in range(LEN):
        print(k, i)
        while numbers[0][0] != i:
            numbers.append(numbers.popleft())

        # pop the number
        n = numbers.popleft()
        pos = n[1] % len(numbers)

        for _ in range(pos):
            numbers.append(numbers.popleft())
        numbers.append(n)

        # print([x[1] for x in numbers])

total = 0
for n in (1000, 2000, 3000):
    zeroi = 0
    while numbers[zeroi][1] != 0:
        zeroi += 1

    x = numbers[(zeroi + n) % LEN][1]
    print(x)
    total += x

print()
print(total)
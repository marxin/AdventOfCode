#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()

items = input.split('\n\n')

def cmp(lhs, rhs):
    while True:
        if not lhs:
            return 0 if not rhs else -1
        elif not rhs:
            return 1

        x = lhs[0]
        y = rhs[0]
        lhs = lhs[1:]
        rhs = rhs[1:]

        if type(x) != type(y):
            if isinstance(x, int):
                x = [x]
            else:
                y = [y]

        if isinstance(x, int) and isinstance(y, int):
            r = x - y
        elif isinstance(x, list) and isinstance(y, list):
            r = cmp(x, y)
        else:
            assert False

        if r != 0:
            return r

suma = 0
for i, item in enumerate(items):
    left, right = [eval(x) for x in item.split()]
    r = cmp(left, right)
    if r < 0:
        suma += i + 1
    # print(i + 1, left, right, r)

print(suma)
#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read().strip()
lines = input.splitlines()

known = {}

for line in lines:
    lhs, rhs = line.split(': ')
    parts = rhs.split()
    if len(parts) == 1:
        known[lhs] = int(rhs)
        print(lhs)

while len(known) != len(lines):
    for line in lines:
        lhs, rhs = line.split(': ')
        if lhs not in known:
            parts = rhs.split()
            arg0 = parts[0]
            arg1 = parts[2]
            if arg0 not in known or arg1 not in known:
                continue

            arg0 = known[arg0]
            arg1 = known[arg1]
            match parts[1]:
                case '+':
                    l = operator.add
                case '-':
                    l = operator.sub
                case '*':
                    l = operator.mul
                case '/':
                    l = operator.floordiv
                case _:
                    assert False
            
            result = l(arg0, arg1)
            known[lhs] = result

print(known['root'])
#!/usr/bin/env python3

import os
import operator
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

H = len(lines)
W = len(lines[0])

monkeys = []


def fn(old, op, arg):
    return op(old, arg if isinstance(arg, int) else old)

common = 1

for chunk in data.split('\n\n'):
    lines = chunk.strip().splitlines()
    m = {}
    m['items'] = [int(x) for x in lines[1].split(':')[-1].split(',')]

    statement = lines[2].split()
    op = operator.add if statement[-2] == '+' else operator.mul
    
    m['lambda'] = (op, 'x' if statement[-1] == 'old' else int(statement[-1]))
    m['div'] = int(lines[3].split()[-1])
    common *= m['div']
    m['throw'] = (int(lines[4].split()[-1]), int(lines[5].split()[-1]))
    m['total'] = 0
    monkeys.append(m)

print(common)
for i in range(10000):
    for m in monkeys:
        newitems = []
        items = list(sorted(m['items']))
        for item in items:
            newitem = fn(item, *m['lambda']) % common
            index = m['throw'][int(newitem % m['div'] != 0)]
            monkeys[index]['items'].append(newitem)
        m['items'] = []
        m['total'] += len(items)

for m in monkeys:
    print(m['total'], m['items'],)

totals = sorted(x['total'] for x in monkeys)
print()
print(totals[-1] * totals[-2])
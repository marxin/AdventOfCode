#!/usr/bin/env python3

import os
import sys

from itertools import permutations
from collections import defaultdict, Counter

DIGITS = {0: 'abcefg', 1: 'cf', 2: 'acdeg', 3: 'acdfg', 4: 'bcdf', 5: 'abdfg', 6: 'abdefg', 7: 'acf', 8: 'abcdefg', 9: 'abcdfg'}
INVERSE = {v: k for k, v in DIGITS.items()}
LETTERS = 'abcdefg'
INLIST = sorted(DIGITS.values())

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

count1478 = 0

for line in lines:
    left, right = line.split(' | ')
    values = right.split()

    for v in values:
        if len(v) in (2, 3, 4, 7):
            count1478 += 1

print(count1478)

def mapnumber(number, mapping):
    return ''.join(sorted([mapping[c] for c in number]))

def get_value(line):
    left, right = line.split(' | ')
    numbers = left.split()
    rhs = right.split()

    for perm in permutations(LETTERS):
        mapping = {}
        for i, l in enumerate(LETTERS):
            mapping[l] = perm[i]
        
        generated = []
        for number in numbers:
            generated.append(mapnumber(number, mapping))
        if sorted(generated) == INLIST:
            v = 0
            for n in rhs:
                v *= 10
                mapped = mapnumber(n, mapping)
                v += INVERSE[mapped]
            return v

total = 0
for i, line in enumerate(lines):
    total += get_value(line)

print(total)
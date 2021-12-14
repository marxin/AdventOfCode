#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()
input = lines[0]

formulas = {}
for line in lines[2:]:
    lhs, rhs = line.split(' -> ')
    formulas[lhs] = rhs

print(input, formulas)

for _ in range(10):
    print(c)
    next = ''
    for i in range(len(input) - 1):
        token = input[i:i + 2]
        if i == 0:
            next = input[0]
        next += formulas[token] + token[1]
    input = next

d = Counter(input)
tuples = sorted(d.items(), key=lambda x: x[1])
print(tuples[-1][1] - tuples[0][1])
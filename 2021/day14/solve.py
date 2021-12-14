#!/usr/bin/env python3

import os
import sys
import math

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

d = {}
for i in range(len(input) - 1):
    token = input[i:i + 2]
    if token not in d:
        d[token] = 0
    d[token] += 1

for _ in range(40):
    d2 = {}
    for k, v in d.items():
        f = formulas[k]
        key1 = k[0] + f
        key2 = f + k[1]
        if key1 not in d2:
            d2[key1] = 0
        if key2 not in d2:
            d2[key2] = 0
        d2[key1] += v
        d2[key2] += v
    d = d2

keys = {}
for k in d:
    keys[k[0]] = 0

for k, v in d.items():
    keys[k[0]] += v
    keys[k[1]] += v

tuples = sorted(keys.items(), key=lambda x: x[1])
small = math.ceil(tuples[0][1] / 2)

big = tuples[-1]
big = math.ceil(big[1] / 2)

print(big - small)
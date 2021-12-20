#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

lines = data.splitlines()
mapping = lines[0]
lines = lines[2:]

data = {}

W = len(lines[0])
H = len(lines)
print(W, H)

for i, line in enumerate(lines):
    for j, c in enumerate(line):
        data[(j, i)] = c

def get_index(x, y):
    s = ''
    for i in range(-1, 2):
        for j in range(-1, 2):
            s += data.get((x + j, y + i), '.')
    return int(s.replace('#', '1').replace('.', '0'), 2)

def print_data(data):
    for y in range(-2, H + 2):
        for x in range(-2, W + 2):
            print(data.get((x, y), '.'), end='')
        print()
    print()

for _ in range(2):
    next_data = {}
    for x in range(-100, W + 100):
        for y in range(-100, H + 100):
            next_data[(x, y)] = mapping[get_index(x, y)]

    print_data(next_data)
    data = next_data

count = 0
for x in range(-2, W + 2):
    for y in range(-2, H + 2):
        if data.get((x, y), '.') == '#':
            count += 1

print(count)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        data[(x, y)] = int(c)

counter = 0
print(len(data.items()))
for key, value in data.items():
    done = True
    for move in ((1, 0), (0, 1), (-1, 0), (0, -1)):
        pos = (key[0] + move[0], key[1] + move[1])
        if pos in data:
            if data[pos] <= value:
                done = False
    if done:
        counter += value + 1

print(counter)
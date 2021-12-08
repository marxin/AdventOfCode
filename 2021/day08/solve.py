#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

count1478 = 0

for line in lines:
    left, right = line.split(' | ')
    values = right.split()
    print(values)

    for v in values:
        if len(v) in (2, 3, 4, 7):
            count1478 += 1

print(count1478)
#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

OPENNING = '([<{'
MAPPING = {')': '(', ']': '[', '>': '<', '}': '{', '(': ')', '[': ']', '<': '>', '{': '}'}
PRICES = {')': 3, ']': 57, '}': 1197, '>': 25137, '(': 1, '[': 2, '{': 3, '<': 4}

price = []

def process_line(line):
    global price
    p = 0

    stack = []
    for c in line:
        if c in OPENNING:
            stack.append(c)
        elif c in MAPPING:
            d = MAPPING[c]
            last = stack.pop()
            if d != last:
                return
        else:
            assert False

    for s in reversed(stack):
        p *= 5
        p += PRICES[s]
    price.append(p)

for line in lines:
    process_line(line)

price.sort()
print(price[len(price) // 2])
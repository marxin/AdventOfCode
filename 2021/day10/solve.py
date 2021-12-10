#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

OPENNING = '([<{'
CLOSING = {')': '(', ']': '[', '>': '<', '}': '{'}
PRICES = {')': 3, ']': 57, '}': 1197, '>': 25137}

price = 0

def process_line(line):
    global price

    stack = []
    for c in line:
        if c in OPENNING:
            stack.append(c)
        elif c in CLOSING:
            d = CLOSING[c]
            last = stack.pop()
            if d != last:
                print('expected', last, 'got', c)
                price += PRICES[c]
                return
        else:
            assert False

for line in lines:
    process_line(line)

print(price)
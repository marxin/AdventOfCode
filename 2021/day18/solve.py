#!/usr/bin/env python3

import os
import sys
import math

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

def parse_line(line):
    for i, c in enumerate(line):
        if c in '[],':
            yield c
        else:
            yield int(c)

def parse(line):
    return list(parse_line(line))

def tostr(tokens):
    return ''.join([str(x) for x in tokens])

def find_explosion_index(tokens):
    opened = 0
    for i, t in enumerate(tokens):
        if opened == 5:
            return i - 1
        if t == '[':
            opened += 1
        elif t == ']':
            opened -= 1
    return -1

def explode(tokens):
    start = find_explosion_index(tokens)
    if start == -1:
        return tokens

    s = tokens[start + 1]
    e = tokens[start + 3]

    for i in range(start, -1, -1):
        if isinstance(tokens[i], int):
            tokens[i] += s
            break        

    for i in range(start + 5, len(tokens)):
        if isinstance(tokens[i], int):
            tokens[i] += e
            break

    return tokens[:start] + [0] + tokens[start + 5:]

def split(tokens):
    for i, v in enumerate(tokens):
        if isinstance(v, int) and v > 9:
            return tokens[:i] + ['[', math.floor(v / 2), ',', math.ceil(v / 2), ']'] + tokens[i + 1:]
    return tokens

def doactions(tokens):
    while True:
        # print(tostr(tokens))
        tokens2 = explode(tokens)
        if tokens == tokens2:
            tokens2 = split(tokens)
            if tokens == tokens2:
                return tokens
            else:
                tokens = tokens2
        else:
            tokens = tokens2

def sum(a, b):
    return ['['] + a + [','] + b + [']']

def calculate_magnitude(tokens):
    while True:
        modified = False
        for i in range(len(tokens) - 2):
            x = tokens[i + 1]
            y = tokens[i + 3]
            if tokens[i] == '[' and isinstance(x, int) and isinstance(y, int):
                tokens = tokens[:i] + [3 * x + 2 * y] + tokens[i + 5:]
                modified = True
                break
        if not modified:
            assert len(tokens) == 1
            return tokens[0]

current = parse(lines[0])
for line in lines[1:]:
    current = sum(current, parse(line))
    current = doactions(current)

print(tostr(current))
print(calculate_magnitude(current))
print('== PART 2 ==')

maximum = 0

for i in range(len(lines)):
    for j in range(len(lines)):
        if i != j:
            a = parse(lines[i])
            b = parse(lines[j])
            c = doactions(sum(a, b))
            m = calculate_magnitude(c)
            if m > maximum:
                maximum = m

print(maximum)
#!/usr/bin/env python3

import re

data = open('input.txt').read()

first, second = data.split('\n\n')

rules = {}
for line in first.splitlines():
    lhs, rhs = line.split(': ')
    parts = rhs.strip('"').split(' ')
    rules[lhs] = parts

data = second.splitlines()

def create_regex(x, y):
    start = rules['0']
    while any([c for c in start if c.isdigit()]):
        start2 = []
        for i, v in enumerate(start):
            if v.isdigit():
                start2.append('(')
                if v == '8' and x > 0:
                    x -= 1
                    start2 += ['42', '8']
                elif v == '11' and y > 0:
                    y -= 1
                    start2 += ['42', '11', '31']
                else:
                    start2 += rules[v]
                start2.append(')')
            else:
                start2 += v
        start = start2
    regex = re.compile(''.join(start))
    return regex

matched = set()

print(len(data))

def measure(regex):
    for line in data:
        if line in matched:
            continue
        m = regex.fullmatch(line)
        if m:
            matched.add(line)

for i in range(0, 10):
    for j in range(0, 10):
        print(i, j)
        regex = create_regex(i, j)
        measure(regex)

print(len(matched))

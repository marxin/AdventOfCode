#!/usr/bin/env python3

import re
import copy

rules = {}
back_rules = {}

lines = open('input.txt').read().splitlines()

for line in lines:
    parts = line.split(' ')
    lhs = parts[0] + ' ' + parts[1]
    assert lhs not in rules
    rules[lhs] = []

    line = ' '.join(parts[3:])
    for m in re.findall('(\d+) ([\w ]+) bag', line):
        rhs = m[1]
        if rhs not in back_rules:
            back_rules[rhs] = set()
        back_rules[rhs].add(lhs)
        rules[lhs].append((m[1], int(m[0])))

#part 1
start = {'shiny gold'}
while True:
    l = len(start)
    for item in list(start):
        if item in back_rules:
            start |= back_rules[item]
    if l == len(start):
        break

print(len(start) - 1)

# part 2
total = 0
bags = {'shiny gold': 1}
while bags:
    print(bags)
    k, v = list(bags.items())[0]
    total += v
    del bags[k]

    if k in rules:
        print(f'{v}x {k} => {rules[k]} (total: {total})')
        for k2, v2 in rules[k]:
            bags.setdefault(k2, 0)
            bags[k2] += v * v2
print(total - 1)

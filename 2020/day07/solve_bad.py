#!/usr/bin/env python3

import re
import copy

rules = {}
cache = {}

lines = open('input.txt').read().splitlines()

for line in lines:
    parts = line.split(' ')
    lhs_color = parts[0] + ' ' + parts[1]
    assert lhs_color not in rules

    line = ' '.join(parts[3:])
    rhs = set()
    for m in re.findall('(\d+) ([\w ]+) bags', line):
        rhs.add(m[1])
    if rhs:
        rules[lhs_color] = rhs

def dfs(bags, depth):
    if 'shiny gold' in bags:
        return True
    fbags = frozenset(bags)
    if fbags in cache:
        return cache[fbags]

    print(bags)

    for color in bags:
        if fbags in cache:
            return cache[fbags]

        if color in rules:
            bags2 = copy.copy(bags)
            bags2 |= rules[color]
            bags2.remove(color)
            r = dfs(bags2, depth + 1)
            if r:
                cache[frozenset(bags2)] = True
                return True
    cache[fbags] = False

first_colors = set(rules.keys())

count = 0
for i, start in enumerate(sorted(first_colors)):
    print(f'{i}/{len(first_colors)}: {start}')
    if dfs({start}, 0):
        print('yep')
        count += 1

print(count)

#!/usr/bin/env python3

import copy
import math

cache = {}

data = open('input.txt').readlines()
for l in data:
    print(l, end='')
print()

steps = 10**9

puzzle = []
for l in data:
    line = []
    for c in l.strip():
        line.append(c)
    puzzle.append(line)

def get_neighbours(x, y):
    d = {}
    for i in range(-1, 2):
        for j in range(-1, 2):
            newx = x + i
            newy = y + j
            if newy >= 0 and newy < len(puzzle):
                if newx >= 0 and newx < len(puzzle[newy]):
                    if newx != x or newy != y:
                        item = puzzle[newy][newx]
                        if not item in d:
                            d[item] = 0
                        d[item] += 1
    return d

s = 0
while s < steps:
    print(s)
    key = str(puzzle)
    if key in cache:
        period = s - cache[key]
        x = math.floor((steps - s) / period)
        if x > 0:
            print(x)
            s += x * period
            print(s)
            continue
    cache[key] = s
    newpuzzle = copy.deepcopy(puzzle)
    for y in range(len(puzzle)):
        for x in range(len(puzzle[0])):
            a = puzzle[y][x]
            d = get_neighbours(x, y)
            replacement = puzzle[y][x]
            if a == '.':
                if '|' in d and d['|'] >= 3:
                    replacement = '|'
            elif a == '|':
                if '#' in d and d['#'] >= 3:
                    replacement = '#'
            elif a == '#':
                if '#' in d and d['#'] >= 1 and '|' in d and d['|'] >= 1:
                    pass
                else:
                    replacement = '.'
            newpuzzle[y][x] = replacement
    puzzle = newpuzzle
    s += 1

wooden = 0
lumber = 0

for l in puzzle:
    for c in l:
        if c == '|':
            wooden += 1
        elif c == '#':
            lumber += 1
        print(c, end='')
    print()

print(wooden * lumber)

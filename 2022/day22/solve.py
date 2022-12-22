#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain, takewhile, dropwhile
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, 1), (-1, 0), (0, -1))

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()

first, second = input.split('\n\n')
lines = first.splitlines()

Y = len(lines)
X = max([len(l) for l in lines])

map = {}
for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c in ('.', '#'):
            map[(x, y)] = c

teleport = dict([(m, {}) for m in MOVES])

for p in map:
    for move in MOVES:
        p2 = (p[0] + move[0], p[1] + move[1])
        if p2 not in map:
            match move:
                case (1, 0):
                    t = min([x for x in map if x[1] == p[1]])
                case (-1, 0):
                    t = max([x for x in map if x[1] == p[1]])
                case (0, 1):
                    t = min([x for x in map if x[0] == p[0]])
                case (0, -1):
                    t = max([x for x in map if x[0] == p[0]])
            teleport[move][p2] = t

start = min([p for p in map if p[1] == 0])
orientation = 0


def parse_commands(line):
    while line:
        cmd = ''.join(takewhile(lambda x: x not in ('L', 'R'), line))
        line = line[len(cmd):]
        yield (int(cmd), line[0] if line else '')
        line = line[1:]


commands = list(parse_commands(second))
print(start)

for i, move in enumerate(commands):    
    steps, rotate = move
    print('Before', i, start, MOVES[orientation], 'steps:', steps)

    for _ in range(steps):
        move = MOVES[orientation]
        start2 = (start[0] + move[0], start[1] + move[1])
        if start2 not in map:
            t = teleport[move][start2]
            if map[t] == '.':
                start2 = t
            else:
                # can't teleport
                break

        if map[start2] == '.':
            start = start2
        else:
            # end the steps
            break

    # rotate
    match rotate:
        case 'R':
            orientation += 1
        case 'L':
            orientation -= 1
        case '':
            pass
    orientation %= len(MOVES) 

print(start, orientation)
p1 = 1000 * (start[1] + 1) + 4 * (start[0] + 1) + orientation
print(p1)
#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain, takewhile, dropwhile
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))
SYMBOLS = ('>', '^', '<', 'v')

FILENAME = 'sample.txt' if True else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()

H = len(lines)
W = len(lines[0])

print(H, W)

start = (1, 0)
end = (W - 2, H - 1)

valid = set()
blizzards = defaultdict(list)

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        p = (x, y)
        if c in SYMBOLS:
            blizzards[p].append(MOVES[SYMBOLS.index(c)])
            valid.add(p)
        elif c == '.':
            valid.add(p)
        else:
            assert c == '#'

print(blizzards)
print(len(valid))


def hash_dict(blizzards):
    r = []
    for k, v in sorted(blizzards.items()):
        r.append((k, tuple(sorted(v))))
    return tuple(r)


def move_blizzards(blizzards):
    blizzards2 = defaultdict(list)

    for p, blizlist in blizzards.items():
        for bliz in blizlist:
            bliz2 = (p[0] + bliz[0], p[1] + bliz[1])
            if bliz2[0] == 0:
                bliz2 = (W - 2, bliz2[1])
            elif bliz2[0] == W - 1:
                bliz2 = (1, bliz2[1])
            elif bliz2[1] == 0:
                bliz2 = (bliz2[0], H - 2)
            elif bliz2[1] == H - 1:
                bliz2 = (bliz2[0], 1)
            blizzards2[bliz2].append(bliz)
            assert bliz2 in valid
    
    return blizzards2


cache = {}
best = sys.maxsize

def visit(p, b, steps):
    global best
    if p == end:
        if steps < best:
            best = steps
            print('New best', best)

    state = (p, hash_dict(b))
    if state in cache and cache[state] <= steps:
        return

    cache[state] = steps

    # wait
    moved = move_blizzards(b)

    if p not in moved:
        visit(p, moved, steps + 1)

    # move in all directions
    for move in MOVES:
        p2 = (p[0] + move[0], p[1] + move[1])
        if p2 in valid and p2 not in moved:
            visit(p2, moved, steps + 1)

visit(start, blizzards, 0)
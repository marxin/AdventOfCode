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

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()

H = len(lines)
W = len(lines[0])

print('H:', H - 2, 'W:', W - 2)

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

# print(blizzards)
# print(len(valid))


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

SCD = 600
# SCD = 12

valid_in_time = {}

b2 = blizzards.copy()
for i in range(SCD):
    print(i)
    valid_in_time[i] = valid - set(b2.keys())
    b2 = move_blizzards(b2)

assert hash_dict(blizzards) == hash_dict(b2)

known = {(start, 0): 0}
worklist = deque([(start, 0)])

while worklist:
    p, b = worklist.popleft()
    steps = known[(p, b)]

    if p == end:
        print('done', steps)
        break

    b = (b + 1) % SCD

    # move in all directions
    for move in MOVES:
        p2 = (p[0] + move[0], p[1] + move[1])
        state = (p2, b)
        if p2 in valid_in_time[b] and state not in known:
            known[state] = steps + 1
            worklist.append((p2, b))

    # wait
    state = (p, b)
    if p in valid_in_time[b] and state not in known:
        known[state] = steps + 1
        worklist.append((p, b))
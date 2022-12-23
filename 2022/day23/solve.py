#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain, takewhile, dropwhile, groupby
from functools import reduce

sys.setrecursionlimit(1000)

# N S W E
MOVES = ((0, -1), (0, 1), (-1, 0), (1, 0))

BOUNDARY = {
    0: ((-1, -1), (0, -1), (1, -1)),
    1: ((-1, 1), (0, 1), (1, 1)),
    2: ((-1, -1), (-1, 0), (-1, 1)),
    3: ((1, -1), (1, 0), (1, 1))
}

bindex = 0

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()

data = set()
H = len(lines)
W = len(lines[0])


def print_set(data):
    minx = min([p[0] for p in data])
    maxx = max([p[0] for p in data])
    miny = min([p[1] for p in data])
    maxy = max([p[1] for p in data])

    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            print('#' if (x, y) in data else '.', end='')
        print()
    print()


for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c == '#':
            data.add((x, y))


def has_neigh(point, data):
    for x in range(-1, 2, 1):
        for y in range(-1, 2, 1):
            if x != 0 or y != 0:
                if (point[0] + x, point[1] + y) in data:
                    return True
    return False


def get_move(point, data, bindex):
    if not has_neigh(point, data):
        return point

    for i in range(len(MOVES)):
        bindex2 = (bindex + i) % len(MOVES)
        can = True
        for move in BOUNDARY[bindex2]:
            point2 = (point[0] + move[0], point[1] + move[1])
            if point2 in data:
                can = False

        if can:
            direction = MOVES[bindex2]
            return (point[0] + direction[0], point[1] + direction[1])

    return point

print_set(data)

i = 0
while True:
    i += 1
    print(f'Step #{i}')

    intentions = {}

    for p in data:
        intentions[p] = get_move(p, data, bindex)

    if data == set(intentions.values()):
        break

    grouping = dict((k, len(list(g))) for k, g in groupby(sorted(intentions.values())))

    data2 = set()

    for p in data:
        p2 = intentions[p]
        if grouping[p2] == 1:
            data2.add(p2)
        else:
            data2.add(p)

    data = data2

    bindex = (bindex + 1) % len(MOVES)
    # print_set(data)

dimx = max([p[0] for p in data]) - min([p[0] for p in data])
dimy = max([p[1] for p in data]) - min([p[1] for p in data])

print((dimx  + 1) * (dimy + 1) - len(data))
print(i)
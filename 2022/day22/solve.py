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

def parse_commands(line):
    while line:
        cmd = ''.join(takewhile(lambda x: x not in ('L', 'R'), line))
        line = line[len(cmd):]
        yield (int(cmd), line[0] if line else '')
        line = line[1:]


commands = list(parse_commands(second))

N = 50
W = 3
H = 4
assert len(map) == 6 * (N * N)


class Face:
    def __init__(self, x, y, map):
        self.x = x
        self.y = y
        self.nr = y * W + x + 1
        self.map = {}

        for p, v in map.items():
            if p[0] // N == self.x and p[1] // N == self.y:
                self.map[(p[0] % N, p[1] % N)] = v

    def __repr__(self):
        return f'[Face(#{self.nr})]'


def printmap(map):
    for y in range(N):
        for x in range(N):
            print(map[(x, y)], end='')
        print()
    print()


faces = {}
for y in range(H):
    for x in range(W):
        f = Face(x, y, map)
        if len(f.map) == N * N:
            faces[f.nr] = f

print(faces)


def rotate_right(p):
    assert 0 <= p[0] < N
    assert 0 <= p[1] < N
    return (N - p[1] - 1, p[0])


def rotate_right_n(p, n):
    for _ in range(n):
        p = rotate_right(p)
    return p


# fill up neighbors
faces[2].neighbors = ( (3, 0), (5, 0), (7, 2), (10, 3) )
faces[3].neighbors = ( (8, 2), (5, 3), (2, 0), (10, 0) )
faces[5].neighbors = ( (3, 1), (8, 0), (7, 1), (2, 0) )
faces[7].neighbors = ( (8, 0), (10, 0), (2, 2), (5, 3) )
faces[8].neighbors = ( (3, 2), (10, 3), (7, 0), (5, 0) )
faces[10].neighbors = ( (8, 1), (3, 0), (2, 1), (7, 0) )

# printmap(faces[2].map)

x = (0, 0)
x = rotate_right(x)
assert x == (49, 0)
x = rotate_right(x)
assert x == (49, 49)
x = rotate_right(x)
assert x == (0, 49)
x = rotate_right(x)
assert x == (0, 0)
assert rotate_right((2, 1)) == (48, 2)

orientation = 0
start = (0, 0)
face = faces[2]

# discard it now
map = None

for i, move in enumerate(commands):
    assert 0 <= start[0] < N
    assert 0 <= start[1] < N

    steps, rotate = move
    print('Before', i, start, MOVES[orientation], move, 'steps:', steps)

    for _ in range(steps):
        move = MOVES[orientation]
        start2 = (start[0] + move[0], start[1] + move[1])
        if start2 not in face.map:
            match orientation:
                case 0:
                    start2 = (0, start2[1])
                case 1:
                    start2 = (start2[0], 0)
                case 2:
                    start2 = (N - 1, start2[1])
                case 3:
                    start2 = (start2[0], N - 1)
            
            newface, newo = face.neighbors[orientation]
            torot = (4 - newo) % len(MOVES)
            start2 = rotate_right_n(start2, torot)
            face2 = faces[newface]
            if face2.map[start2] == '.':
                start = start2
                face = face2
                orientation = (orientation + torot) % len(MOVES)
            else:
                # end the steps
                break
        else:
            if face.map[start2] == '.':
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

print(start)
start = (start[0] + face.x * N, start[1] + face.y * N)
print(start)

p2 = 1000 * (start[1] + 1) + 4 * (start[0] + 1) + orientation
print(p2)
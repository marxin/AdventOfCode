#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

SHAPE_INPUT = '''
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
'''

shapes = []

for part in SHAPE_INPUT.split('\n\n'):
    part = part.strip()
    s = set()
    for y, line in enumerate(part.split()):
        for x, c in enumerate(line):
            if c == '#':
                s.add((x, y))
    shapes.append(s)

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()

WIDTH = 7
LEFT = 2
UP = 3

pixels = set((x, 0) for x in range(WIDTH))
print('Shapes', len(shapes), ', moves', len(input))

sidx = 0
moveidx = 0


def get_min_y(pixels):
    return min([x[1] for x in pixels])

def get_max_y(pixels):
    return max([x[1] for x in pixels])


def in_touch(pixels, shape):
    for s in shape:
        moved = (s[0], s[1] + 1)
        if moved in pixels:
            return True
    return False


def move(pixels, shape, shift):
    shape2 = set([(x[0] + shift[0], x[1] + shift[1]) for x in shape])

    # test boundary collision
    for p in shape2:
        if p[0] < 0 or p[0] >= WIDTH:
            return shape.copy()
        if p in pixels:
            return shape.copy()

    # all is fine, we can move it
    return shape2


def print_pixels(pixels, shape=set()):
    miny = get_min_y(pixels | shape)
    pixels2 = set((x[0], x[1] - miny) for x in pixels)
    shape2 = set((x[0], x[1] - miny) for x in shape)

    for y in range(-miny + 1):
        for x in range(WIDTH):
            p = (x, y)
            if p in shape2:
                print('@', end='')
            else:
                print('#' if p in pixels2 else '.', end='')
        print()
    print()

for i in range(2022):
    shape = shapes[sidx]
    start = (LEFT, get_min_y(pixels) - UP - get_max_y(shape) - 1)
    shape = move(pixels, shape, start)
    sidx = (sidx + 1) % len(shapes)

    # print(f'Before {i + 1}')
    # print_pixels(pixels, shape)
    # move until there is no contact
    while True:
        m = (1,0) if input[moveidx] == '>' else (-1, 0)
        moveidx = (moveidx + 1) % len(input)
        shape = move(pixels, shape, m)
        if in_touch(pixels, shape):
            pixels |= shape
            break
        else:
            shape = move(pixels, shape, (0, 1))

# print_pixels(pixels)
print(get_max_y(pixels) - get_min_y(pixels))
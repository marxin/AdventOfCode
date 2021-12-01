#!/usr/bin/env python3

moves = [(0, -1), (1, 0), (0, 1), (-1, 0)]
directions = 'NESW'
rotations = [0, 90, 180, 270]

orientation = 1
start = (0, 0)

def move(line):
    global start
    global orientation
    c = line[0]
    value = int(line[1:])
    if c in directions:
        i = directions.index(c)
        m = moves[i]
        start = (start[0] + value * m[0], start[1] + value * m[1])
    elif c == 'F':
        m = moves[orientation]
        start = (start[0] + value * m[0], start[1] + value * m[1])
    elif c == 'L':
        d = rotations.index(value)
        orientation = (orientation - d) % 4
    elif c == 'R':
        d = rotations.index(value)
        orientation = (orientation + d) % 4
    else:
        assert False

data = open('input.txt').read()
lines = data.splitlines()

for line in lines:
    move(line)
    print(f'{start}: {orientation}')

print(f'{abs(start[0]) + abs(start[1])}')

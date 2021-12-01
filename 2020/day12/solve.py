#!/usr/bin/env python3

moves = [(0, -1), (1, 0), (0, 1), (-1, 0)]
directions = 'NESW'
rotations = [0, 90, 180, 270]

rotation_transform = [
        lambda w: (-w[1], w[0]),
        lambda w: (-w[1], w[0]),
        lambda w: (-w[0], -w[1]),
        lambda w: (w[1], -w[0]),
    ]

orientation = 1
start = (0, 0)
waypoint = (10, -1)

def move(line):
    global start
    global waypoint
    global orientation
    c = line[0]
    value = int(line[1:])
    if c in directions:
        i = directions.index(c)
        m = moves[i]
        waypoint= (waypoint[0] + value * m[0], waypoint[1] + value * m[1])
    elif c == 'F':
        start = (start[0] + value * waypoint[0], start[1] + value * waypoint[1])
    elif c == 'L':
        d = -rotations.index(value) % 4
        print(d)
        waypoint = rotation_transform[d](waypoint)
    elif c == 'R':
        d = rotations.index(value) % 4
        print(d)
        waypoint = rotation_transform[d](waypoint)
    else:
        assert False

data = open('input.txt').read()
lines = data.splitlines()

for line in lines:
    move(line)
    print(f'{line}: {start}: {waypoint}: {orientation}')

print(f'{abs(start[0]) + abs(start[1])}')

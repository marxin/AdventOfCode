#!/usr/bin/env python3

data = open('input.txt').read()

d = {
        '^': (0, 1),
        'v': (0, -1),
        '<': (-1, 0),
        '>': (1, 0)
}

positions = [(0, 0), (0, 0)]
seen = {(0, 0)}

for i, c in enumerate(data.rstrip()):
    pos = positions[i % 2]
    move = d[c]
    pos = (pos[0] + move[0], pos[1] + move[1])
    positions[i % 2] = pos
    seen.add(pos)

print(len(seen))

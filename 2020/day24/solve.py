#!/usr/bin/env python3

directions = ['e', 'se', 'sw', 'w', 'nw', 'ne']
moves = [(1, 0), (0.5, -0.5), (-0.5, -0.5), (-1, 0), (-0.5, 0.5), (0.5, 0.5)]

data = open('input.txt').read()
lines = data.splitlines()

seen = set()

for line in lines:
    start = (0, 0)
    while line:
        d = line[0]
        line = line[1:]
        if not d in directions:
            d += line[0]
            line = line[1:]
        index = directions.index(d)
        move = moves[index]
        start = (start[0] + move[0], start[1] + move[1])
    if start in seen:
        seen.remove(start)
    else:
        seen.add(start)

def neighbours(tile):
    c = 0
    for m in moves:
        tile2 = (tile[0] + m[0], tile[1] + m[1])
        if tile2 in seen:
            c += 1
    return c

print(seen)
print(len(seen))
print()

for i in range(100):
    candidates = set()
    seen2 = set()

    for tile in seen:
        for m in moves:
            candidates.add((tile[0] + m[0], tile[1] + m[1]))

    for tile in seen:
        n = neighbours(tile)
        if n == 0 or n > 2:
            pass
        else:
            seen2.add(tile)

    for cand in candidates:
        if cand not in seen:
            n = neighbours(cand)
            if n == 2:
                seen2.add(cand)
    print(len(seen2))
    seen = seen2

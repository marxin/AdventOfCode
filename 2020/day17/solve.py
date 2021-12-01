#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

lights = set()

N = len(lines)

for i in range(N):
    for j in range(N):
        value = lines[j][i]
        if value == '#':
            lights.add((j, i, 0, 0))

def neighbors(point):
    for i in range(-1, 2):
        for j in range(-1, 2):
            for k in range(-1, 2):
                for l in range(-1, 2):
                    if i == 0 and j == 0 and k == 0 and l == 0:
                        pass
                    else:
                        yield (point[0] + i, point[1] + j, point[2] + k, point[3] + l)

def step():
    global lights
    lights_next = set()
    candidates = set()
    for l in lights:
        n = set(neighbors(l))
        candidates |= n
        length = len([x for x in n if x in lights])
        if length == 2 or length == 3:
            lights_next.add(l)

    for c in candidates:
        n = set(neighbors(c))
        length = len([x for x in n if x in lights])
        if length == 3:
            lights_next.add(c)
    lights = lights_next

print(data)

n = list(neighbors((0, 0, 0, 0)))
assert (0, 0, 0, 0) not in n

for i in range(6):
    step()

print(len(lights))

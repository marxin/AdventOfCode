#!/usr/bin/env python3

import copy

data = open('input.txt').read()
lines = data.splitlines()

data = {}

X = len(lines[0])
Y = len(lines)

for i in range(Y):
    for j in range(X):
        data[(j, i)] = lines[i][j]

MOVES = []
for i in range(-1, 2):
    for j in range(-1, 2):
        if i != 0 or j != 0:
            MOVES.append((i, j))

def adjacent(d, pos):
    moves = copy.copy(MOVES)
    a = 0
    c = 1
    while moves:
        for m in list(moves):
            x = pos[0] + c * m[0] 
            y = pos[1] + c * m[1]
            if x < 0 or x >= X or y < 0 or y >= Y:
                moves.remove(m)
                continue
            if d[(x, y)] == '#':
                a += 1
                moves.remove(m)
            elif d[(x, y)] == 'L':
                moves.remove(m)
        c += 1
    assert a <= 8
    return a

def step(d):
    d2 = {}
    for i in range(Y):
        for j in range(X):
            pos = (j, i)
            if d[pos] != '.':
                a = adjacent(d, pos)
                if d[pos] == 'L' and a == 0:
                    d2[pos] = '#'
                elif d[pos] == '#' and a >= 5:
                    d2[pos] = 'L'
                else:
                    d2[pos] = d[pos]
            else:
                d2[pos] = '.'
    return d2

def printme(d):
    for i in range(Y):
        for j in range(X):
            print(d[(j, i)], end='')
        print()
    print()

steps = 0
while True:
    printme(data)
    steps += 1
    prev = data
    data = step(data)
    if prev == data:
        break

print(steps)
occupied = len([x for x in data.values() if x == '#'])
print(f'occupied: {occupied}')

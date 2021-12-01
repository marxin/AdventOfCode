#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

data = {}

X = len(lines[0])
Y = len(lines)

for i in range(Y):
    for j in range(X):
        data[(j, i)] = lines[i][j]

def adjacent(d, pos):
    a = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i != 0 or j != 0:
                x = pos[0] + i
                y = pos[1] + j
                if x >= 0 and x < X and y >= 0 and y < Y and d[(x, y)] == '#':
                    a += 1
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
                elif d[pos] == '#' and a >= 4:
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
    steps += 1
    prev = data
    data = step(data)
    if prev == data:
        break

print(steps)
occupied = len([x for x in data.values() if x == '#'])
print(f'occupied: {occupied}')

#!/usr/bin/env python3

d = {}

data = open('input.txt').read()
lines = data.splitlines()

width = len(lines[0])
height = len(lines)

def neig(position):
    n = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i != 0 or j != 0:
                x = position[0] + i
                y = position[1] + j
                if x >= 0 and x < width and y >= 0 and y < height:
                    if d[(x, y)] == '#':
                        n += 1
    return n

def fix():
    global d
    d[(0, 0)] = '#'
    d[(width - 1, 0)] = '#'
    d[(0, height - 1)] = '#'
    d[(width - 1, height - 1)] = '#'

def step():
    global d
    d2 = {}
    for i in range(height):
        for j in range(width):
            pos = (j, i)
            v = d[pos]
            n = neig(pos)
            if v == '#':
                if n == 2 or n == 3:
                    d2[pos] = '#'
                else:
                    d2[pos] = '.'
            else:
                d2[pos] = '#' if n == 3 else '.'
    d = d2
    fix()

def printme():
    for i in range(height):
        for j in range(width):
            pos = (j, i)
            print(d[pos], end='')
        print()
    print()

for i in range(height):
    for j in range(width):
        d[(j, i)] = lines[i][j]
fix()

printme()
for i in range(100):
    step()
    printme()

print(len([x for x in d.values() if x == '#']))

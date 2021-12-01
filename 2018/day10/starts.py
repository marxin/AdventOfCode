#!/usr/bin/env python3

import re
import matplotlib.pyplot as plt

points = []

for l in open('input.txt').readlines():
    m = re.match('position=<(.+), (.+)> velocity=<(.+),(.+)>.*', l)    
    points.append([int(x) for x in m.groups()])

N = 10000
moves = ((1, 0), (-1, 0), (0, 1), (0, -1))

def get_points(n):
    moved = set()
    for p in points:
        x = p[0] + n * p[2]
        y = p[1] + n * p[3]
        moved.add((x, y))
    return moved

def get_neighbors(moved):
    c = 0
    for m in moved:
        for i in range(len(moves)):
            newpos = (m[0] + moves[i][0], m[1] + moves[i][1])
            if newpos in moved:
                c += 1
    return c

N = 10027
D = 0
for n in range(N - D, N + D + 1):
    moved = get_points(n)
    x = []
    y = []
    for m in moved:
        x.append(m[0])
        y.append(m[1])
    print('%d: %d' % (n, get_neighbors(moved)))
    plt.scatter(x, y, s=0.6)
    plt.savefig('%d.png' % n)
    plt.close()

for i in range(100, 150):
    for j in range(140, 250):
        c = '#' if (j, i) in moved else ' '
        print(c, end = '')
    print()

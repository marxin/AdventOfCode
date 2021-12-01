#!/usr/bin/env python3

import re

lines = [l.strip() for l in open('input.txt').readlines()]

claims = []

for l in lines:
    m = re.match('#(\d+) @ (\d+),(\d+): (\d+)x(\d+)', l)
    claims.append([int(x) for x in m.groups()[1:]])

N = 1000
matrix = [[0] * N for x in range(N)]

for i, c in enumerate(claims):
    x0 = c[0]
    y0 = c[1]
    for x in range(c[2]):
        for y in range(c[3]):
            matrix[x0 + x][y0 + y] += 1

def no_overlap(c):
    x0 = c[0]
    y0 = c[1]
    for x in range(c[2]):
        for y in range(c[3]):
            if matrix[x0 + x][y0 + y] > 1:
                return False
    return True

more = 0
for i in range(N):
    for j in range(N):
        if matrix[i][j] >= 2:
            more += 1

print(more)

for i, c in enumerate(claims):
    if no_overlap(c):
        print(i + 1)

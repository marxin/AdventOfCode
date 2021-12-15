#!/usr/bin/env python3

import os
import sys
import heapq

from collections import defaultdict, Counter

sys.setrecursionlimit(50000)

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

d = {}

X = len(lines[0])
Y = len(lines)
print(X, Y)

for y, line in enumerate(lines):
    for x, v in enumerate(line):
        d[(x, y)] = int(v)

start = (0, 0)
end = (X - 1, Y - 1)
best = sys.maxsize
cache = {}
counter = 0

def distance(pos):
    return X - pos[0] + Y - pos[1]

def heuristics(pos, length):
    length += d[pos]
    return length

heap = [(-1, (start, 0))]
heapq.heapify(heap)

while heap:
    _, (pos, length) = heapq.heappop(heap)

    counter += 1
    if counter % 100000 == 0:
        print(best, length, len(cache))

    if length >= best:
        if not pos in cache:
            cache[pos] = length
        continue
    elif pos in cache:
        if cache[pos] <= length:
            continue
    else:
        cache[pos] = length

    if pos == end:
        print(length)
        best = length
        continue
    else:
        for m in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            pos2 = (pos[0] + m[0], pos[1] + m[1])
            if pos2 in d:
                heapq.heappush(heap, (heuristics(pos2, length), (pos2, length + d[pos2])))
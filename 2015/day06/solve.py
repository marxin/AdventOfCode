#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

N = 1000
d = {}
for i in range(N):
    for j in range(N):
        d[(i, j)] = 0

for l in lines:
    parts = l.split(' ')
    if 'turn on' in l:
        fn = lambda x: x + 1
    elif 'turn off' in l:
        fn = lambda x: max(0, x - 1)
    elif 'toggle' in l:
        fn = lambda x: x + 2

    start = parts[-3]
    end = parts[-1]

    x1, y1 = [int(x) for x in start.split(',')]
    x2, y2 = [int(x) for x in end.split(',')]

    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            d[(x, y)] = fn(d[(x, y)])

print(sum(d.values()))

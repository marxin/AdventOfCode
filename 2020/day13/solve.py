#!/usr/bin/env python3

import math
import sys

data = open('input.txt').read()
l0, l1 = data.splitlines()

offset = int(l0)
buses = [int(x) for x in l1.split(',') if x != 'x']

print(offset)

minimum = sys.maxsize
minimum_bus = None

for b in buses:
    n = math.ceil(offset / b)
    diff = n * b - offset
    if diff < minimum:
        minimum = diff
        minimum_bus = b

print(minimum * minimum_bus)

print('===')

def smallen(x, y):
    for b in buses:
        if b > 0 and x % b == 0 and y % b == 0:
            x = int(x / b)
            y = int(y / b)
    return (x, y)

def common(x, x0, y, y0, d):
    ox = x
    ox0 = x0
    oy = y
    oy0 = y0
    print(x, x0, y, y0, d)
    x0, y0 = smallen(x0, y0)
    print(f'Smallen to {x0}, {y0}')

    while True:
        if x + d == y and (x - ox) % ox0 == 0:
            return (x, x0 * y0)
        if x + d < y:
            x += x0
        else:
            N = math.floor((x + d - y) / y0)
            y += max(N, 1) * y0

print(buses)
print()

tuples = []
B0 = buses[0]

buses = [int(x) if x != 'x' else 0 for x in l1.split(',')]
for i in range(1, len(buses)):
    if buses[i] == 0:
        continue
    k = common(buses[0], buses[0], buses[i], buses[i], i)
    print(f'({buses[0]}, {buses[i]}, {i}): {k}')
    tuples.append(k)


print('xxx')
newtuples = []

x, x0 = tuples[0]
print(tuples)

for i in range(1, len(tuples)):
    y, y0 = tuples[i]
    x, x0 = common(x, x0, y, y0, 0)

print(x, x0)

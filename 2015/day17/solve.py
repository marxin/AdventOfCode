#!/usr/bin/env python3

data = open('input.txt').read()
lines = [int(x) for x in data.splitlines()]

SUM = 150
total = 0

for i in range(2 ** len(lines)):
    popcount = bin(i).count('1')
    if popcount != 4:
        continue
    s = 0
    for j in range(20):
        if i & (1 << j):
            s += lines[j]
    if s == SUM:
        total += 1

print(total)

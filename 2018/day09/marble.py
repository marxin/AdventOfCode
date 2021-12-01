#!/usr/bin/env python3

from collections import deque

values = deque([0])

N = 70953 * 100
P = 405
p = 0

players = [0] * P

for i in range(1, N + 1):
    p = (p + 1) % P
    if i % 23 == 0:
        players[p] += i
        values.rotate(7)
        players[p] += values.pop()
        values.rotate(-1)
    else:
        values.rotate(-1)
        values.append(i)

print(players)
print(max(players))

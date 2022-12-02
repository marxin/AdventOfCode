#!/usr/bin/env python3

import os

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

data = {}
H = len(lines)
W = len(lines[0])


def get_result(a, b):
    if a == b:
        return 3
    elif a == 0 and b == 1:
        return 6
    elif a == 1 and b == 2:
        return 6
    elif a == 2 and b == 0:
        return 6
    else:
        return 0


total = 0
games = []
for y, line in enumerate(lines):
    a, b = line.split()
    a = ord(a) - ord('A')
    b = ord(b) - ord('X')
    games.append((a, b))
    points = get_result(a, b) + b + 1
    total += points

print(total)

# part 2
total2 = 0
for a, b in games:
    me = 0
    for i in range(3):
        if get_result(a, i) == b * 3:
            me = i
            break
    total2 += get_result(a, me) + me + 1

print(total2)

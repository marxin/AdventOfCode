#!/usr/bin/env python3

import os

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

position = [0, 0, 0]

for line in lines:
    cmd, value = line.split()
    value = int(value)

    if cmd == 'forward':
        position[0] += value
        position[1] += value * position[2]
    elif cmd == 'down':
        position[2] += value
    elif cmd == 'up':
        position[2] -= value

print(position)
print(position[0] * position[1])
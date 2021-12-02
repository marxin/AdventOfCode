#!/usr/bin/env python3

import os

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

position = [0, 0]

for line in lines:
    cmd, value = line.split()
    value = int(value)

    if cmd == 'forward':
        position[0] += value
    elif cmd == 'down':
        position[1] += value
    elif cmd == 'up':
        position[1] -= value

print(position)
print(position[0] * position[1])
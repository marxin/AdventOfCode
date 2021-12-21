#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

pos = [int(line.split()[-1]) for line in lines]
score = [0,0]

cube = 1
times = 0

def get_shift():
    global cube, times
    times += 3
    s = 0
    for _ in range(3):
        s += cube
        cube += 1
        if cube > 100:
            cube = 1
    return s    

done = False
THRESHOLD = 1000

while not done:
    for i in range(2):
        pos[i] += get_shift()
        if pos[i] % 10 == 0:
            pos[i] = 10
        else:
            pos[i] %= 10
        score[i] += pos[i]
        if score[i] >= 1000:
            done = True
            break

print(times)
print(min(score) * times)
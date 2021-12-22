#!/usr/bin/env python3

import os
import sys
import copy

import collections
from collections import defaultdict, Counter
from heapq import *

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

start = [int(line.split()[-1]) for line in lines]
pos = start.copy()
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

print('Part1:', min(score) * times)

dices = defaultdict(int)

N = 3

for i in range(1, N + 1):
    for j in range(1, N + 1):
        for k in range(1, N + 1):
            dices[i + j + k] += 1

start = (0, 0, *start, 0)
THRESHOLD = 21

cache = {}

def investigate(state):
    assert len(state) == 5
    if state in cache:
        return cache[state]

    who = state[4]

    if state[0] >= THRESHOLD:
        return (1, 0)
    elif state[1] >= THRESHOLD:
        return (0, 1)

    suma = (0, 0)
    for dice, times in dices.items():
        value = list(state).copy()
        v = value[who + 2]
        v += dice
        if v > 10:
            v -= 10
        assert v <= 10
        value[who] += v
        value[who + 2] = v

        value[4] = (who + 1) % 2
        state2 = tuple(value)
        p0, p1 = investigate(state2)
        suma = (suma[0] + times * p0, suma[1] + times * p1)
    cache[state] = suma
    return suma

r = investigate(start)
print(r)
print(max(r))
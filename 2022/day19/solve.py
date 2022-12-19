#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

types = ('ore', 'clay', 'obsidian', 'geode')

blueprints = []

for i, line in enumerate(lines):
    needs = {}
    parts = [x.strip().split() for x in line.split('Each')]
    needs[0] = {0: int(parts[1][3])}
    needs[1] = {0: int(parts[2][3])}
    needs[2] = {0: int(parts[3][3]), 1: int(parts[3][-2])}
    needs[3] = {0: int(parts[4][3]), 2: int(parts[4][-2])}
    blueprints.append(needs)

TIME = 24
GEODE = 3


def canbuy(robot, mined):
    n = sys.maxsize
    for k, v in robot.items():
        n = min(mined[k] // v, n)
    return n

counter = 0
hits = 0
nohits = 0

def get_maximum(cache, blueprint, robots, mined, time, maximum):
    global counter, hits, nohits
    counter += 1
    if counter % 100000 == 0:
        print(counter, len(cache), hits / nohits)
        print(time, robots, mined, maximum[0])

    key = (tuple(robots.items()), tuple(mined.items()), time)
    if key in cache:
        hits += 1
        return cache[key]
    else:
        nohits += 1
    
    if time == 0:
        if mined[GEODE] > maximum[0]:
            maximum[0] = mined[GEODE]
            print('New maximum', maximum)
        cache[key] = mined[GEODE]
        return mined[GEODE]
    
    # mine first
    for robot, n in robots.items():
        mined[robot] += n

    best = 0

    # try to buy a robot
    for robot, values in blueprint.items():
        n = canbuy(values, mined)
        for i in range(0, n + 1):
            robots2 = robots.copy()
            mined2 = mined.copy()

            robots2[robot] += i
            for x, y in values.items():
                mined2[x] -= i * y
                assert mined2[x] >= 0
            
            b = get_maximum(cache, blueprint, robots2, mined2, time - 1, maximum)
            if b > best:
                best = b
            
    b = get_maximum(cache, blueprint, robots.copy(), mined.copy(), time - 1, maximum)
    if b > best:
        best = b

    cache[key] = best
    return best


for i, blueprint in enumerate(blueprints):
    print(blueprint)
    get_maximum({}, blueprint, defaultdict(int, {0: 1}), defaultdict(int), TIME, [0])
    asdf

print(blueprints)
#!/usr/bin/env python3

import os
import sys
import concurrent.futures

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

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
MAXPOP = 6

# TRY
MAXPOP = 20

def canbuy(robot, mined):
    for k, v in robot.items():
        if mined[k] < v:
            return False
    return True

def mine(robots, mined):
    for robot, n in robots.items():
        mined[robot] += n

def get_maximum(cache, blueprint, robots, mined, time, maximum):

    key = (tuple(robots.items()), tuple(mined.items()), time)
    t = cache.get(key)
    if t and t >= time:
        return

    if mined[0] > 10:
        return

    if time == 0:
        if mined[GEODE] > maximum[0]:
            maximum[0] = mined[GEODE]
            # print('.. new maximum', maximum[0], robots, max(robots.values()))
        return
    
    # try to buy a robot
    for robot in range(GEODE, -1, -1):
        values = blueprint[robot]
        if robots[robot] > MAXPOP:
            continue

        if canbuy(values, mined):
            robots2 = robots.copy()
            mined2 = mined.copy()
            
            for x, y in values.items():
                mined2[x] -= y
                assert mined2[x] >= 0

            mine(robots2, mined2)            
            robots2[robot] += 1

            get_maximum(cache, blueprint, robots2, mined2, time - 1, maximum)

    mine(robots, mined)
    get_maximum(cache, blueprint, robots, mined, time - 1, maximum)
    cache[key] = time


def work(i, blueprint):
    print('start', i)
    best = [0]
    get_maximum({}, blueprint, defaultdict(int, {0: 1}), defaultdict(int), TIME, best)
    print(i, best[0])
    return i * best[0]


total = 0
futures = []

with concurrent.futures.ProcessPoolExecutor() as executor:
    for i, blueprint in enumerate(blueprints):
        futures.append(executor.submit(work, i + 1, blueprint))

    for future in concurrent.futures.as_completed(futures):
        total += future.result()

print(total)
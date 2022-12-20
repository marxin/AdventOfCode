#!/usr/bin/env python3

import os
import sys
import concurrent.futures
import operator

from collections import Counter
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

N = 4
TIME = 32
GEODE = 3

# TODO: hack2
MAXPOP = 15

def canbuy(robot, mined):
    for k, v in robot.items():
        if mined[k] < v:
            return False
    return True

def mine(robots, mined):
    for i in range(N):
        mined[i] += robots[i]


def is_cache_better(cached, mined):
    for i in range(N):
        if cached[i] < mined[i]:
            return False
    return True

counter = 0
hits = 0

def get_maximum(cache, blueprint, maxmines, robots, mined, time, maximum):
    global counter, hits
    counter += 1

    if counter % 10 ** 6 == 0:
        print(counter, len(cache), hits / len(cache))

    key = (tuple(robots), tuple(mined), time)
    if key in cache:
        hits += 1
        return

    # TODO: hack
    if all(maxmines[i] <= mined[i] for i in range(N - 1)):
        cache.add(key)
        return

    if time == 0:
        if mined[GEODE] > maximum[0]:
            maximum[0] = mined[GEODE]
            print('.. new maximum', maximum[0], robots, 'cache len', len(cache))
        return
    
    # try to buy a robot
    for robot in range(GEODE, -1, -1):
        values = blueprint[robot]
        if robots[robot] > MAXPOP:
            continue

        if robot != GEODE and robots[robot] >= maxmines[robot]:
            continue

        if canbuy(values, mined):
            robots2 = robots.copy()
            mined2 = mined.copy()
            
            for x, y in values.items():
                mined2[x] -= y
                assert mined2[x] >= 0

            mine(robots2, mined2)            
            robots2[robot] += 1

            get_maximum(cache, blueprint, maxmines, robots2, mined2, time - 1, maximum)

    mine(robots, mined)
    get_maximum(cache, blueprint, maxmines, robots, mined, time - 1, maximum)
    cache.add(key)


def work(i, blueprint, maxmines):
    global counter, hits
    counter = 0
    hits = 0
    print('start', i)
    best = [0]
    get_maximum(set(), blueprint, maxmines, [1, 0, 0, 0], [0] * N, TIME, best)
    print(i, best[0])
    return best[0]


futures = []

with concurrent.futures.ProcessPoolExecutor(max_workers=1) as executor:
    for i, blueprint in enumerate(blueprints):
        maxmines = [0] * (N - 1)
        for needs in blueprint.values():
            for key, value in needs.items():
                maxmines[key] = max(maxmines[key], value)
        print('Max mines', maxmines)
        futures.append(executor.submit(work, i + 1, blueprint, maxmines))

    results = []
    for future in concurrent.futures.as_completed(futures):
        results.append(future.result())

print(results)

print(reduce(operator.mul, results))
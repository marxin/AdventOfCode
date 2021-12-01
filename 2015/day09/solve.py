#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

cities = set()
routes = {}

best = 0

def find_minimal_path(seen, city, total):
    global best
    if len(seen) == len(cities):
        if total > best:
            best = total
            print(best)
    for c in cities - seen:
        key = (city, c)
        if key in routes:
            seen.add(c)
            find_minimal_path(seen, c, total + routes[key])
            seen.remove(c)

for line in lines:
    f, _, t, _, distance = line.split(' ')
    routes[(f, t)] = int(distance)
    routes[(t, f)] = int(distance)
    cities.add(f)
    cities.add(t)

for city in cities:
    find_minimal_path({city}, city, 0)

#!/usr/bin/env python3

import copy

data = open('input.txt').read()
lines = data.splitlines()

d = {}
names = []

for line in lines:
    parts = line.split(' ')
    personA = parts[0]
    personB = parts[-1][:-1]
    units = int(parts[3])
    if parts[2] == 'lose':
        units = -units
    d[(personA, personB)] = units
    d[(personA, 'ME')] = 0
    d[('ME', personA)] = 0
    if not personA in names:
        names.append(personA)

names.append('ME')
allperms = []

def permutations(available, order):
    global allperms
    if not available:
        allperms.append(copy.copy(order))
        return

    for a in available:
        order.append(a)
        available.remove(a)
        permutations(available, order)
        available.add(a)
        order.pop()

def evaluate(permutation):
    total = 0
    for i in range(len(permutation)):
        j = (i + 1) % len(names)
        x = d[(permutation[i], permutation[j])]
        y = d[(permutation[j], permutation[i])]
        total += (x + y)
    return total

print(d)
permutations(set(names), [])

best = -10**10
for perm in allperms:
    r = evaluate(perm)
    if r > best:
        best = r

print(best)

#!/usr/bin/env python3

import os
import sys
import heapq

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

MOVES = ((0, 1), (0, -1), (1, 0), (-1, 0))
NOSTOP = set([(3 + 2 * x, 1) for x in range(4)])
ATOMS = -1
floor = set()
atoms = {}

def get_energy(c):
    if not c.isalpha():
        return None
    return 10 ** (ord(c) - ord('A'))

def get_my_column(c):
    assert c.isalpha()
    distance = ord(c) - ord('A')
    return 3 + distance * 2

def is_house_position(pos):
    return pos[1] >= 2

def printmaze(atoms, steps):
    print()
    print(steps)
    for y in range(7):
        for x in range(13):
            pos = (x, y)
            if pos in atoms:
                print(atoms[pos], end='')
            elif pos in floor:
                print('.', end='')
            else:
                print('#', end='')
        print()

IN_ROW = 4

def all_below_ok(pos, c, atoms):
    if pos[1] == IN_ROW + 1:
        return True
    for y in range(pos[1] + 1, IN_ROW + 2):
        pos2 = (pos[0], y)
        if pos2 not in atoms or atoms[pos2] != c:
            return False
    return True

def get_reachable(pos, atoms):
    origpos = pos
    queue = [pos]
    c = atoms[pos]
    mycol = get_my_column(c)
    if pos[0] == mycol:
        if all_below_ok(pos, c, atoms):
            return {}
    
    leaving_house = is_house_position(pos)

    energy = get_energy(c)
    reachable = {pos: 0}
    del atoms[pos]

    while queue:
        pos = queue.pop()
        for move in MOVES:
            pos2 = (pos[0] + move[0], pos[1] + move[1])
            if pos2 in floor and pos2 not in atoms and pos2 and pos2 not in reachable:
                reachable[pos2] = reachable[pos] + energy
                queue.append(pos2)
    atoms[origpos] = c

    # Remove forbidden configurations
    for nostop in NOSTOP:
        reachable.pop(nostop, None)
    del reachable[origpos]

    if leaving_house:
        for pos2 in list(reachable.keys()):
            if is_house_position(pos2):
                del reachable[pos2]
    else:
        for pos2 in list(reachable.keys()):
            if pos2[0] == mycol:
                if not all_below_ok(pos2, c, atoms):
                    del reachable[pos2]
            else:
                del reachable[pos2]

    return reachable

def atoms_home(atoms):
    home = 0
    for pos, atom in atoms.items():
        if pos[0] == get_my_column(atom):
            home += 1
    return home

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        pos = (x, y)
        if get_energy(c):
            atoms[pos] = c
            floor.add(pos)
        elif c == '.':
            floor.add(pos)

print(floor)
print(atoms)

ATOMS = len(atoms)

best = sys.maxsize
best_to_home = sys.maxsize

cache = {}
heap = []
heapq.heapify(heap)
heapq.heappush(heap, ((ATOMS, 0, 0), atoms))

counter = 0
i = 0
while heap:
    i += 1    
    (tohome, steps, _), atoms = heapq.heappop(heap)

    if tohome < best_to_home:
        best_to_home = tohome

    if i % 100000 == 0:
        print(i, len(heap), len(cache), best_to_home)
        # printmaze(atoms, steps)

    key = tuple(sorted(atoms.items()))
    if key in cache:
        if steps < cache[key]:
            cache[key] = steps
        else:
            continue
    else:
        cache[key] = steps

    if steps > best:
        continue

    if tohome == 0:
        if steps < best:
            best = steps
            print(f'New best: {steps}')
    else:
        for pos, atom in list(atoms.items()):
            reachable = get_reachable(pos, atoms)
            for r, steps2 in reachable.items():
                atoms2 = atoms.copy()
                del atoms2[pos]
                atoms2[r] = atom
                assert len(atoms2) == ATOMS
                counter += 1
                heapq.heappush(heap, ((ATOMS - atoms_home(atoms2), steps + steps2, counter), atoms2))
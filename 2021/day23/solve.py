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
ATOMS = 8
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
    for y in range(5):
        for x in range(13):
            pos = (x, y)
            if pos in atoms:
                print(atoms[pos], end='')
            elif pos in floor:
                print('.', end='')
            else:
                print('#', end='')
        print()

def get_reachable(pos, atoms):
    origpos = pos
    queue = [pos]
    c = atoms[pos]
    mycol = get_my_column(c)
    if pos[0] == mycol:
        pos2 = (pos[0], pos[1] + 1)
        if pos[1] == 3 or (pos2 in atoms and atoms[pos2] == c):
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
            if pos2[0] == mycol:
                continue
            elif pos2[1] == origpos[1] or is_house_position(pos2):
                del reachable[pos2]
    else:
        for pos2 in list(reachable.keys()):
            if is_house_position(pos2) and pos2[0] == mycol:
                if pos2[1] == 2:
                    pos3 = (pos2[0], pos2[1] + 3)
                    if pos3 in atoms and atoms[pos3] != 'c':
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

best = sys.maxsize
cache = {}
heap = []
heapq.heapify(heap)
heapq.heappush(heap, ((ATOMS, 0, 0), atoms))

counter = 0
while heap:
    (tohome, steps, _), atoms = heapq.heappop(heap)

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
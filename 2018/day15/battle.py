#!/usr/bin/env python3

from collections import deque
from termcolor import colored
import sys

directions = ((0, -1), (-1, 0), (1, 0), (0, 1))

def get_unit_by_position(units, position):
    for u in units:
        if u.position == position:
            return u
    return None

class Unit:
    def __init__(self, position, type, unit_id):
        self.position = position
        self.type = type
        self.hp = 200
        self.attack = 3
        self.id = unit_id
        pass

    def __repr__(self):
        return '%s: %s: %d' % (self.type, str(self.position), self.hp)

    def get_color(self):
        return 'red' if self.type == 'G' else 'green'

    def get_closest_enemy(self, map, units):
        enemies = dict([(u.position, [u, sys.maxsize]) for u in units if u.type != self.type])
        flood = {}
        flood[self.position] = 0
        queue = deque([self.position])
        while len(queue):
            p = queue.popleft()
            for move in directions:
                newpos = (p[0] + move[0], p[1] + move[1])
                if not newpos in map:
                    steps = flood[p] + 1
                    if newpos in enemies and enemies[newpos][1] > steps:
                        enemies[newpos][1] = steps
                    if not newpos in flood and not get_unit_by_position(units, newpos):
                        flood[newpos] = steps
                        queue.append(newpos)
        return list(sorted(enemies.values(), key = lambda x: x[1]))[0]

    def try_one_move(self, map, units, start, distance):
        flood = {}
        flood[start] = 0
        queue = deque([start])
        while len(queue):
            p = queue.popleft()
            for move in directions:
                newpos = (p[0] + move[0], p[1] + move[1])
                if not newpos in map:
                    steps = flood[p] + 1
                    if not newpos in flood and not get_unit_by_position(units, newpos):
                        flood[newpos] = steps
                        queue.append(newpos)
        for move in directions:
            newpos = (self.position[0] + move[0], self.position[1] + move[1])
            if newpos in flood and flood[newpos] == distance - 1:
                return newpos
        return None

    def move_to_closest_enemy(self, map, units, closest, distance):
        for move in directions:
            newpos = (closest.position[0] + move[0], closest.position[1] + move[1])
            if not newpos in map and get_unit_by_position(units, newpos) == None:
                np = self.try_one_move(map, units, newpos, distance - 1)
                if np:
                    return np
        return None

    def move(self, map, units):
        closest = self.get_closest_enemy(map, units)
        distance = closest[1]
        if distance == sys.maxsize:
            return
        print('Closest for %s%d -> %s%d (distance: %d)' % (self.type, self.id, closest[0].type, closest[0].id, distance))
        if distance > 1:
            newpos = self.move_to_closest_enemy(map, units, closest[0], distance)
            if newpos != None:
                self.position = newpos

    def fight(self, map, units):
        candidates = []
        for move in directions:
            newpos = (self.position[0] + move[0], self.position[1] + move[1])
            unit = get_unit_by_position(units, newpos)
            if unit and unit.type != self.type:
                candidates.append(unit)

        candidates = list(sorted(candidates, key = lambda x: (x.hp, x.order())))
        if candidates:
            unit = candidates[0]
            assert unit.hp > 0
            unit.hp -= self.attack
            if unit.hp <= 0:
                units.remove(unit)

    def order(self):
        return list(reversed(self.position))

lines = open('input.txt').readlines()
map = set()
units = []

def print_map(rounds):
    print('\nAfter %d rounds:' % rounds)
    for y in range(len(lines)):
        for x in range(len(lines[0]) - 1):
            t = (x, y)
            c = None
            if t in map:
                c = '#'
            else:
                u = get_unit_by_position(units, t)
                if u:
                    c = colored(u.type, u.get_color())
                else:
                    c = '.'
            print(c, end = '')
        u = sorted([x for x in units if x.position[1] == y], key = lambda x: x.position[0])
        print('   ', end='')
        for x in u:
            print(' %s(%d)' % (colored(x.type + str(x.id), x.get_color()), x.hp), end = '')
        print()

unit_id = 0
for y, l in enumerate(lines):
    l = l.strip()
    for x, c in enumerate(l):
        if c == '#':
            map.add((x, y))
        elif c == 'G' or c == 'E':
            units.append(Unit((x, y), c, unit_id))
            unit_id += 1

print_map(0)
for i in range(1, 500):
    for unit in list(sorted(units, key = lambda x: x.order())):
        types = set([u.type for u in units])
        if len(types) == 1:
            print('End of the fight!')
            hps = sum([u.hp for u in units])
            print(hps)
            print(i - 1)
            print(hps * (i - 1))
            exit(0)
        if unit.hp <= 0:
            assert not unit in units
            continue
        unit.move(map, units)
        unit.fight(map, units)

    print_map(i)

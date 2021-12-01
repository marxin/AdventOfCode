#!/usr/bin/env python3

weapons, armor, rings = open('input.txt').read().split('\n\n')

def parse(lines):
    for line in lines:
        parts = [x for x in line.split(' ') if x]
        yield (int(parts[-3]), int(parts[-2]), int(parts[-1]))

weapons = weapons.splitlines()[1:]
weapons = list(parse(weapons))

armor = armor.splitlines()[1:]
armor = list(parse(armor))

rings = rings.splitlines()[1:]
rings = list(parse(rings))

groups = (weapons, armor, rings)
print(groups)

me2 = [100, 0, 0]
boss2 = [100, 8, 2]

def play(p0, p1):
    hit = max(p0[1] - p1[2], 1)
    p1[0] -= hit

def match(me, boss):
    while True:
        play(me, boss)
        if boss[0] <= 0:
            return True
        play(boss, me)
        if me[0] <= 0:
            return False

mincost = 100000
maxcost = 0
for weapon in weapons:
    print(weapon)
    me = me2.copy()
    boss = boss2.copy()
    for a in armor:
        for r in rings:
            for r2 in rings:
                me[0] = 100
                me[1] = weapon[1] + r[1] + r2[1]
                me[2] = weapon[2] + r[2] + r2[2]
                cost = weapon[0] +  r[0] + r2[0]
                if not match(me, boss2.copy()):
                    if cost > maxcost:
                        maxcost = cost
                        print('max', cost)
                        print(me, boss)

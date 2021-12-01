#!/usr/bin/env python3

orbits = []

class Planet:
    def __init__(self, name):
        self.name = name
        self.parent = None

for o in open('input.txt').readlines():
    orbits.append(o.strip().split(')'))

planets = {}

for o in orbits:
    pname1 = o[0]
    pname2 = o[1]
    if not pname1 in planets:
        planets[pname1] = Planet(pname1)
    if not pname2 in planets:
        planets[pname2] = Planet(pname2)
    assert planets[pname2].parent == None
    planets[pname2].parent = planets[pname1]

total = 0
for (_, planet) in planets.items():
    p = planet
    while p.parent:
        total += 1
        p = p.parent

print(total)

print('YOU')
p = planets['YOU']
i = 0
d = {}
while p.parent:
    print(p.parent.name)
    d[p.parent.name] = i
    i += 1
    p = p.parent

p = planets['SAN']
i = 0
while p.parent:
    print(p.parent.name)

    if p.parent.name in d:
        print('Done at %s: %d + %d' % (p.parent.name, i, d[p.parent.name]))
        exit(0)
    i += 1
    p = p.parent

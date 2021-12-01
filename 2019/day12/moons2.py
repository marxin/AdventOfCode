#!/usr/bin/env python3

import re

class Planet:
    def __init__(self, x, y, z):
        self.position = [int(x), int(y), int(z)]
        self.velocity = [0, 0, 0]
        self.total_position = 0
        self.total_velocity = 0

    def calculate_gravity(self, planets):
        for p in planets:
            if p != self:
                for i in range(3):
                    diff = self.position[i] - p.position[i]
                    if diff < 0:
                        self.velocity[i] += 1
                    elif diff > 0:
                        self.velocity[i] += -1

    def apply_gravity(self):
        for i in range(3):
            self.position[i] += self.velocity[i]

    def get_energy(self):
        for i in range(3):
            self.total_position += abs(self.position[i])
            self.total_velocity += abs(self.velocity[i])
        return self.total_position * self.total_velocity

    def eq(self, other):
        for i in range(3):
            if self.position[i] != other.position[i]:
                return False
            if self.velocity[i] != other.velocity[i]:
                return False
        return True

    def print(self):
        print('pos=%-16svel:%s' % (str(self.position), str(self.velocity)))

planets = []
planets2 = []

lines = [x.strip() for x in open('input.txt').readlines()]
for l in lines:
    m = re.match(r'<x=(.*), y=(.*), z=(.*)>', l)
    planets.append(Planet(m.group(1), m.group(2), m.group(3)))
    planets2.append(Planet(m.group(1), m.group(2), m.group(3)))

for planet in planets:
    planet.print()

seen = set()

step = 0
while True:
    step += 1
    print('After step #%d, seen=%d' % (step, len(seen)))
    for planet in planets:
        planet.calculate_gravity(planets)
    for planet in planets:
        planet.apply_gravity()
    home = 0

    positions = []
    for p in planets:
        positions.append(tuple(p.position))
        #print(p.print())
    positions = tuple(positions)
    #print(positions)
    seen.add(positions)

    for i, p in enumerate(planets):
        if p.eq(planets2[i]):
            home += 1
            print('Step #%d: planet is at home:' % step)
            print(p.print())
    if home == len(planets):
        exit(0)

print(sum([p.get_energy() for p in planets]))
print(get_dump(planets))

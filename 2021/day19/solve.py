#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

R = 24

def roll(v):
    return (v[0],v[2],-v[1])

def turn(v):
    return (-v[1],v[0],v[2])

def sequence(v):
    for cycle in range(2):
        for step in range(3):  # Yield RTTT 3 times
            v = roll(v)
            yield(v)           #    Yield R
            for i in range(3): #    Yield TTT
                v = turn(v)
                yield(v)
        v = roll(turn(roll(v)))  # Do RTR

def delta(v0, v1):
    return (v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2])

def add(v0, v1):
    return (v1[0] + v0[0], v1[1] + v0[1], v1[2] + v0[2])

def neg(v):
    return [-v[0], -v[1], -v[2]]

class Scanner:
    def __init__(self, data):
        lines = data.splitlines()
        self.vectors = [tuple([int(x) for x in line.split(',')]) for line in lines[1:]]
        self.initialize()

    def initialize(self):
        self.vectorsset = set(self.vectors)
        self.rotations = []
        for v in self.vectors:
            f = list(sequence(v))
            assert v in f
            assert len(f) == R
            assert len(f) == len(set(f))
            self.rotations.append(f)

    def to_global(self, d, rotation):
        self.vectors = [add(d, self.get(i, rotation)) for i in range(len(self.vectors))]
        self.initialize()

    def get(self, nth, rotation):
        return self.rotations[nth][rotation]

    def find_overlaps(self, other, rotation):        
        N = len(other.vectors)
        M = len(self.vectors)        

        for v0 in self.vectors[:M - 11]:
            for i in range(N):
                v1 = other.get(i, rotation)
                d = add(v0, neg(v1))
                moved = set()
                for j in range(N):
                    moved.add(add(d, other.get(j, rotation)))
                intersection = self.vectorsset & moved
                if len(intersection) >= 12:
                    return (d, rotation)
        return None

    def find_all_overlaps(self, other):
        for r in range(R):
            done = self.find_overlaps(other, r)
            if done:
                return done
        return None


folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()


scanners = [Scanner(part) for part in data.split('\n\n')]

seen = set([0])
tested = set()

while len(seen) != len(scanners):
    for i in range(len(scanners)):
        for j in range(len(scanners)):
            if i in seen and j not in seen and (i, j) not in tested:
                print(i, j, 'seen:', len(seen))
                tested.add((i, j))
                r = scanners[i].find_all_overlaps(scanners[j])
                if r:
                    # print(i, j, r)
                    scanners[j].to_global(*r)
                    seen.add(j)

all = set()
for s in scanners:
    all |= set(s.vectors)

print(len(all))
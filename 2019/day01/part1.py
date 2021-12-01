#!/usr/bin/env python3

from math import floor

def fuel(weight):
    r = floor(weight / 3) - 2
    return r if r > 0 else 0

def fuel_recursive(weight):
    total = 0
    while weight != 0:
        f = fuel(weight)
        total += f
        weight = f
    return total

values = [int(l) for l in open('part1.txt').readlines()]

#for v in values:
#    print('%d:%d' % (v, fuel(v)))

print(sum([fuel(v) for v in values]))

#print(fuel_recursive(1969))
#print(fuel_recursive(100756))
print(sum([fuel_recursive(v) for v in values]))

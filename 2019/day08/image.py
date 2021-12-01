#!/usr/bin/env python3

from textwrap import wrap

data = open('input.txt').read().strip()
width = 25
height = 6

layers = wrap(data, width * height)

min = 2**32
layer = None

for l in layers:
    zeros = len([x for x in l if x == '0'])
    if zeros < min:
        min = zeros
        layer = l

print(min)
print(layer)

ones = len([x for x in layer if x == '1'])
twos = len([x for x in layer if x == '2'])

print(ones * twos)

#!/usr/bin/env python3

import os

def build(lines, bits, bigger):
    result = ''
    for i in range(bits):
        vals = [l[i] for l in lines]
        zeros, ones = vals.count('0'), vals.count('1')
        one = zeros < ones
        if not bigger:
            one = not one
        result += '1' if one else '0'
    return result

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

bits = len(lines[0])

a = build(lines, bits, True)
b = build(lines, bits, False)
print(int(a, 2) * int(b, 2))

#!/usr/bin/env python3

import os

def filter(lines, one):
    lines = lines.copy()
    for i in range(len(lines[0]) + 1):
        bits = [x[i] for x in lines]
        ones, zeros = bits.count('1'), bits.count('0')
        if one:
            s = '1' if ones >= zeros else '0'
        else:
            s = '0' if ones >= zeros else '1'
        lines = [l for l in lines if l[i] == s]
        if len(lines) == 1:
            return lines[0]       


folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

a = filter(lines, True)
b = filter(lines, False)

print(int(a, 2) * int(b, 2))
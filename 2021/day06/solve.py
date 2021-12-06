#!/usr/bin/env python3

import os

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

d = {}

for x in lines[0].split(','):
    x = int(x)
    if x not in d:
        d[x] = 0
    d[x] += 1

for i in range(256):
    d2 = {6: 0}
    for k, v in d.items():
        if k == 0:
            d2[8] = v
            d2[6] += v
        else:
            k2 = k - 1
            if k2 not in d2:
                d2[k2] = 0
            d2[k2] += v
    d = d2

print(d.values())
print(sum(d.values()))

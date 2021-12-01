#!/usr/bin/env python3

import sys

value = 20151125
position = [1, 1]
d = 1

while True:
    if position == [2981, 3075]:
        print(value)
        sys.exit(0)
    value = (value * 252533) % 33554393
    if position[0] == 1:
        d += 1
        position = [d, 1]
    else:
        position[0] -= 1
        position[1] += 1

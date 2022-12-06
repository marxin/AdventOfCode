#!/usr/bin/env python3

import os
import sys

from itertools import product, permutations
from collections import defaultdict, Counter

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

def find_position(string, width):
    for i in range(len(string) - width + 1):
        chunk = set(string[i:i + width])
        if len(chunk) == width:
            return i + width
    assert None


print(find_position(data, 4))
print(find_position(data, 14))
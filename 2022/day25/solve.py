#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain, takewhile, dropwhile
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read()
lines = input.splitlines()


def parse(line):
    n = 0
    for i, c in enumerate(reversed(line)):
        match c:
            case '=':
                x = -2
            case '-':
                x = -1
            case _:
                x = int(c)
        n += x * 5 ** i
    return n


def encrypt(number):
    result = ''

    while number:
        rem = number % 5
        match rem:
            case 3:
                result += '='
                number += 5
            case 4:
                result += '-'
                number += 5
            case _:
                result += str(rem)
        number //= 5

    return result[::-1]


numbers = [parse(n) for n in lines]
print(numbers)
total = sum(numbers)

print(total)
print(encrypt(total))
assert total == parse(encrypt(total))
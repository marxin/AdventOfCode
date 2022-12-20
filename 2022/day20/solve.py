#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))
KEY = 0
KEY = 811589153


folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, 'input.txt')).read()
lines = input.splitlines()

numbers = []


class Number:
    def __init__(self, n):
        self.n = n
        self.wrap = n


for y, line in enumerate(lines):
    n = int(line)
    numbers.append(Number(n * KEY))

LEN = len(numbers)
assert len(set(numbers)) == LEN

for n in numbers:
    n.wrap = n.n % LEN
    if n.n < 0:
        n.wrap -= LEN
    elif n.n > LEN:
        n.wrap += LEN
        print(n.n, n.wrap)

print([(x.n, x.wrap) for x in numbers])

def find_num(numbers, num):
    for i in range(LEN):
        if numbers[i] == num:
            return i
    assert False


def norm(x):
    return x % LEN


def norm2(x):
    y = x % LEN
    if x < 0:
        y -= LEN
    if x >= LEN:
        y += LEN
    return y


candidates = numbers.copy()
print([x.n for x in numbers])

for k in range(1):
    for j, n in enumerate(candidates):
        print(k, j)
        i = find_num(numbers, n)
        i2 = i + n.wrap

        if i2 > i:
            for j in range(i, i2):
                numbers[norm(j)] = numbers[norm(j + 1)]
        else:
            for j in range(i, i2, -1):
                numbers[norm(j)] = numbers[norm(j - 1)]
        numbers[norm(i2)] = n
        print([x.n for x in numbers])

index = 0
while True:
    if numbers[index].n == 0:
        break
    index += 1

print([x.n for x in numbers])
print()
print('zero index', index)

total = 0
for i in (1000, 2000, 3000):
    i2 = index + i
    print(numbers[norm(i2)].n)

    total += numbers[norm(i2)].n

print('result', total)
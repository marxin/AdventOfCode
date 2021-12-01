#!/usr/bin/env python3

r = range(236491,713787+1)

def getdigits(i):
    digits = []
    while i != 0:
        digits.append(i % 10)
        i = int(i / 10)
    return list(reversed(digits))

def has_adjacent(digits):
    for i in range(len(digits) - 1):
        if digits[i] == digits[i + 1]:
            return True
    return False

def non_decreasing(digits):
    for i in range(len(digits) - 1):
        if digits[i] > digits[i + 1]:
            return False
    return True

def valid(digits):
    if not has_adjacent(digits):
        return False
    if not non_decreasing(digits):
        return False
    return True

def split(digits):
    groups = []
    for d in digits:
        if len(groups) == 0 or groups[-1][0] != d:
            groups.append([d])
        else:
            groups[-1].append(d)
    return groups

def even_groups(groups):
    for g in groups:
        l = len(g)
        if l == 2:
            return True
    return False

s1 = 0
s2 = 0

for i in r:
    parts = getdigits(i)
    if valid(parts):
        s1 += 1
        groups = split(parts)
        print(i)
        if even_groups(groups):
            print(groups)
            s2 += 1

print()
print(s1)
print(s2)

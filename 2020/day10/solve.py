#!/usr/bin/env python3

lines = open('input.txt').read().splitlines()

numbers = sorted([int(n) for n in lines])

END = numbers[-1] + 3
nset = set(numbers) | {END}

diffs = {1: 1, 3: 1}

for i, n in enumerate(numbers[:-1]):
    diffs[numbers[i + 1] - n] += 1

print(diffs)
print(diffs[1] * diffs[3])

print('===')
print(END)

cache = {}

def solve(n):
    if n == END:
        return 1 
    elif n in cache:
        return cache[n]
    else:
        local = 0
        for i in range(1, 4):
            n2 = n + i
            if n2 in nset:
                local += solve(n2)
        cache[n] = local
        return local

print(solve(0))

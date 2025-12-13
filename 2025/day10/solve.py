#!/usr/bin/env python3

import z3
from functools import reduce

lines = open("input.txt").read().splitlines()
total = 0


def find(lines, joltage, suma):
    KB = [z3.Int(f"K{i}") for i in range(0, len(keyboards))]
    for i, j in enumerate(joltage):
        expr = None
        for kx, k in enumerate(keyboards):
            if i in k:
                if expr is not None:
                    expr = expr + KB[kx]
                else:
                    expr = KB[kx]
        s.add(expr == j)

    for k in KB:
        s.add(k >= 0)

    total = reduce(lambda expr, k: expr + k, KB)
    s.add(total <= suma)
    s.check()
    m = s.model()
    return sum([m[k].as_long() for k in KB])


suma = 0
for line in lines:
    s = z3.Solver()
    parts = line.split(" ")[1:]
    joltage = [int(x) for x in parts[-1][1:-1].split(",")]
    keyboards = [[int(y) for y in x[1:-1].split(",")] for x in parts[:-1]]

    best = 10**10
    try:
        for i in range(400, 0, -1):
            if i >= best:
                continue
            best = find(lines, joltage, i)
    except Exception:
        print(best)
        suma += best

print(suma)

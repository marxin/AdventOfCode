#!/usr/bin/env python3

s = 0
seen = set()

values = [int(x) for x in open('input.txt').readlines()]
i = 0
while True:
    s += values[i]
    if s in seen:
        print(s)
        exit(0)
    else:
        seen.add(s)
    i = (i + 1) % len(values)

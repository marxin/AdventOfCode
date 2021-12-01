#!/usr/bin/env python3

data = open('input.txt').read()
lines = [int(x) for x in data.splitlines()]

def increase_count(values):
    increased = 0

    for i, v in enumerate(values):
        if i > 0 and v > values[i - 1]:
            increased += 1
    return increased

print(increase_count(lines))

transformed = []
for i, v in enumerate(lines):
    if i + 2 < len(lines):
        transformed.append(v + lines[i + 1] + lines[i + 2])

print(increase_count(transformed))
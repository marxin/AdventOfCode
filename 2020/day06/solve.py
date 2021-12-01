#!/usr/bin/env python3

chunks = open('input.txt').read().split('\n\n')

print(sum([len(set(c.replace('\n', ''))) for c in chunks]))

suma = 0
for chunk in chunks:
    parts = chunk.splitlines()
    allyes = set(parts[0])
    for part in parts:
        allyes &= set(part)

    suma += len(allyes)

print(suma)

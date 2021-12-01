#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

s = 0

for line in lines:
    s += len(line)
    s -= len(eval(line))

print(s)

s2 = 0
for line in lines:
    s2 += line.count('"') + line.count('\\') + 2

print(s2)

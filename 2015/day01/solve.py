#!/usr/bin/env python3

data = open('input.txt').read()

print(data.count('(') - data.count(')'))

floor = 0
for i, x in enumerate(data):
    if x == '(':
        floor += 1
    elif x == ')':
        floor -= 1
    else:
        assert False
    if floor == -1:
        print(i + 1)
        break

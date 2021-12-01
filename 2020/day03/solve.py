#!/usr/bin/env python3

data = [l.strip() for l in open('input.txt').readlines()]

def traverse(x, y):
    pos = [0, 0]
    trees = 0
    width = len(data[0])
    height = len(data)

    while pos[1] < height:
        if data[pos[1]][pos[0]] == '#':
            trees += 1
        pos[0] += x
        pos[1] += y

        if pos[0] >= width:
            pos[0] %= width

    return trees

print(traverse(3, 1))

multiplied = 1
for x, y in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)):
    multiplied *= traverse(x, y)
print(multiplied)

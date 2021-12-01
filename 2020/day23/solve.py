#!/usr/bin/env python3

class Cup:
    def __init__(self, value):
        self.value = value
        self.right = None

    def link(self, other):
        assert self != other
        other.right = self.right
        self.right = other

    def __repr__(self):
        return str(self.value)

data = [3, 8, 9,  1,  2,  5,  4,  6,  7]
data = [5, 9, 8, 1, 6, 2, 7, 3, 4]

while len(data) != 1000 * 1000:
    data.append(len(data) + 1)

cups = [Cup(c) for c in data]
value_to_cup = {}
root = cups[0]

for i, c in enumerate(cups):
    value_to_cup[c.value] = c
    if i + 1 >= len(cups):
        cups[len(cups) - 1].right = cups[0]
    else:
        c.right = cups[i + 1]

def find_smaller(value, chunk):
    chunk_values = [x.value for x in chunk]
    value -= 1
    while True:
        if value == 0:
            value = len(data)
        if not value in chunk_values:
            return value
        value -= 1

def printlist():
    p = root
    for i in range(len(cups)):
        print(f'{p} ', end='')
        p = p.right
    print()

def move():
    global root
    chunk = [root.right.right.right, root.right.right, root.right]
    # printlist()
    # print(chunk)
    root.right = chunk[0].right

    smaller = find_smaller(root.value, chunk)
    pivot = value_to_cup[smaller]

    for c in chunk:
        pivot.link(c)
    root = root.right

for i in range(10 * 1000 * 1000):
    if i % 100000 == 0:
        print(i)
    move()
    # printlist()

v0 = value_to_cup[1].right.value
v1 = value_to_cup[1].right.right.value

print(v0, v1, v0 * v1)

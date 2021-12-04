#!/usr/bin/env python3

from itertools import takewhile, dropwhile

import os
import sys

class Board:
    def __init__(self, lines):
        self.boards = lines

    def done(self, line, seen):
        return all([x in seen for x in line])

    def entire_line(self, seen):
        for b in self.boards:
            if self.done(b, seen):
                return b
        for i in range(len(self.boards[0])):
            line = [l[i] for l in self.boards]
            if self.done(line, seen):
                return line
        return None

    def sum_unmarked(self, seen):
        s = 0
        for b in self.boards:
            for v in b:
                if v not in seen:
                    s += v
        return s

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

boards = []
numbers = [int(x) for x in lines[0].split(',')]

lines = lines[2:]

while lines:
    board = list([[int(n) for n in l.split()] for l in takewhile(lambda x: x, lines)])
    boards.append(Board(board))
    lines = lines[len(board) + 1:]

print(len(boards))
seen = set()
for n in numbers:
    seen.add(n)
    for board in boards:
        el = board.entire_line(seen)
        if el:
            print(el)
            print(board.sum_unmarked(seen) * n)
            sys.exit(0)
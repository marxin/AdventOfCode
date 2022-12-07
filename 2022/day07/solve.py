#!/usr/bin/env python3

import os
import sys

from itertools import takewhile


class Folder:
    def __init__(self, name, parent):
        self.name = name
        self.files = 0
        self.dirs = {}
        self.parent = parent

    def get_size(self):
        size = self.files
        for child in self.dirs.values():
            size += child.get_size()
        return size

    def walk(self, fn):
        fn(self)
        for child in self.dirs.values():
            child.walk(fn)


folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

root = Folder('/', None)
cwd = root

LIMIT = 100000
TOTAL = 70000000
NEEDED = 30000000

i = 0
while i < len(lines):
    line = lines[i]
    assert line.startswith('$')
    parts = line.split()

    match parts[1]:
        case 'cd':
            match parts[2]:
                case '/':
                    cwd = root
                case '..':
                    cwd = cwd.parent
                case _:
                    cwd = cwd.dirs[parts[2]]
        case 'ls':
            items = list(takewhile(lambda x: not x.startswith('$'), lines[i + 1:]))
            i += len(items)
            for item in items:
                first, second = item.split()
                if first == 'dir':
                    cwd.dirs[second] = Folder(second, cwd)
                else:
                    cwd.files += int(first)
        case _:
            assert False
    i += 1

total = 0


def count_total(node):
    global total
    size = node.get_size()
    if size <= LIMIT:
        total += size
        # print(node.name, size)


best = sys.maxsize
torelease = NEEDED - (TOTAL - root.get_size())


def find_smallest(node):
    global best
    size = node.get_size()
    if size >= torelease and size < best:
        best = size


root.walk(count_total)
print(total)

root.walk(find_smallest)
print(best)
#!/usr/bin/env python3

import os
import sys
import operator

from collections import defaultdict, Counter, deque
from itertools import product, permutations, chain
from functools import reduce

sys.setrecursionlimit(1000)

MOVES = ((1, 0), (0, -1), (-1, 0), (0, 1))

FILENAME = 'sample.txt' if False else 'input.txt'

folder = os.path.dirname(os.path.abspath(__file__))
input = open(os.path.join(folder, FILENAME)).read().strip()
lines = input.splitlines()


class Node:
    def __init__(self, line):
        lhs, rhs = line.split(': ')
        parts = rhs.split()

        self.lhs = lhs
        if len(parts) == 1:
            self.value = int(rhs)
        else:
            self.value = None
            self.op = parts[1]
            self.arg0 = parts[0]
            self.arg1 = parts[2]

            self.left = None
            self.right = None

    def __repr__(self):
        return f'{self.lhs} [{self.value}]'

    def human_in_subtree(self):
        if self.lhs == 'humn':
            return True
        elif self.value is not None:
            return False
        return self.left.human_in_subtree() or self.right.human_in_subtree()

    def get_op(self):
        match self.op:
            case '+':
                return operator.add
            case '-':
                return operator.sub
            case '*':
                return operator.mul
            case '/':
                return operator.floordiv
            case _:
                assert False

    def evaluate(self):
        if self.value is not None:
            return self.value
        else:
            return self.get_op()(self.left.evaluate(), self.right.evaluate())

    def find(self, C):
        if self.lhs == 'humn':
            print('DONE', C)
            sys.exit(0)
        
        inleft = self.left.human_in_subtree()
        inright = self.right.human_in_subtree()
        assert inleft != inright

        if inleft:
            B = self.right.evaluate()
            match self.op:
                case '+':
                    self.left.find(C - B)
                case '-':
                    self.left.find(C + B)
                case '*':
                    self.left.find(C // B)
                case '/':
                    self.left.find(C * B)
        else:
            A = self.left.evaluate()
            match self.op:
                case '+':
                    self.right.find(C - A)
                case '-':
                    self.right.find(A - C)
                case '*':
                    self.right.find(C // A)
                case '/':
                    self.right.find(A // C)



nodes = {}

for line in lines:
    n = Node(line)
    nodes[n.lhs] = n

for n in nodes.values():
    if n.value is None:
        n.left = nodes[n.arg0]
        n.right = nodes[n.arg1]

root = nodes['root']
start = root.left if root.left.human_in_subtree() else root.right
val = root.right.evaluate() if start == root.left else root.left.evaluate()
print(start, val)
print(nodes['root'].evaluate())

print(start.find(val))

sys.exit(0)

def calculate(human):
    known = {}
    worklist = set()

    for line in lines:
        lhs, rhs = line.split(': ')
        parts = rhs.split()
        if len(parts) == 1:
            known[lhs] = int(rhs)
            if lhs == 'humn':
                known[lhs] = human
        else:
            worklist.add(line)

    while worklist:
        for line in list(worklist):
            lhs, rhs = line.split(': ')
            if lhs not in known:
                parts = rhs.split()
                arg0 = parts[0]
                arg1 = parts[2]
                if arg0 not in known or arg1 not in known:
                    continue

                arg0 = known[arg0]
                arg1 = known[arg1]
                if parts[1] == '+':
                        l = operator.add
                elif parts[1] == '-':
                        l = operator.sub
                elif parts[1] == '*':
                        l = operator.mul
                else:
                        l = operator.floordiv
                
                if lhs == 'root':
                    print(arg0, arg1)
                    print(line)
                    known[lhs] = arg0 == arg1
                else:
                    result = l(arg0, arg1)
                    known[lhs] = result
                worklist.remove(line)

    return known['root']


for i in range(100000):
    print(i)

    r = calculate(i)
    if r:
        print(i)
        break
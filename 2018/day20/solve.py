#!/usr/bin/env python3

import os
import sys

import sys
sys.setrecursionlimit(15000)

from itertools import product, permutations
from collections import defaultdict, Counter, deque

from sympy import FallingFactorial

class Or:
    def __init__(self, parts):
        self.parts = []
        for p in parts:
            print(len(p))
            if len(p) < 100:
                print(p)
            element = list(parse(p))
            self.parts.append(element)

    def __repr__(self):
        return 'Or(' + ''.join(str(x) for x in self.parts) + ')'

def parse(line):
    start = None
    if '(' in line:
        start = line.index('(')
    if '|' in line:
        start2 = line.index('|')
        if not start or start2 < start:
            start = start2

    if start is None:
        if line:
            yield line
        return

    if line[start] == '(':
        if start != 0:
            before = line[:start]
            yield before
            line = line[start:]

        paren = 0
        for i in range(len(line)):
            if line[i] == '(':
                paren += 1
            elif line[i] == ')':
                paren -= 1
            if paren == 0:
                p1 = list(parse(line[1:i]))
                p2 = list(parse(line[i + 1:]))
                if p1:
                    yield p1
                if p2:
                    yield p2
                return
    elif line[start] == '|':
        parts = []

        paren = 0
        while line:
            for i in range(len(line)):
                if line[i] == '|' and paren == 0:
                    parts.append(line[:i])
                    line = line[i + 1:]
                    break
                if line[i] == '(':
                    paren += 1
                elif line[i] == ')':
                    paren += 1

                if i == len(line) - 1:
                    parts.append(line)
                    line = ''
        yield Or(parts)
    else:
        assert False

MOVES = {
    'N': (0, 1),
    'E': (1, 0),
    'S': (0, -1),
    'W': (-1, 0)
}

nodes = None
edges = None

def walk(pos, tokens, final=True):
    nodes.add(pos)

    if not tokens:
        return pos

    token = tokens[0]
    tokens = tokens[1:]

    if isinstance(token, str):
        for c in token:
            m = MOVES[c]
            prev = pos
            pos = (pos[0] + m[0], pos[1] + m[1])
            nodes.add(pos)
            edges.add((prev, pos))
            edges.add((pos, prev))
        walk(pos, tokens, True)
    elif isinstance(token, Or):
        orig = pos
        for option in token.parts:
            pos = walk(orig, option, False)
            walk(pos, tokens)
    elif isinstance(token, list):
        walk(pos, token + tokens)
    return pos

start = (0, 0)

def get_distance(path):
    global nodes, edges

    nodes = set()
    edges = set()
    tokens = list(parse(path))
    walk(start, tokens)

    flood = {start: 0}
    todo = deque([start])
    while todo:
        pos = todo.popleft()
        steps = flood[pos]
        
        for move in MOVES.values():
            pos2 = (pos[0] + move[0], pos[1] + move[1])
            if (pos, pos2) in edges:
                if pos2 in flood:
                    assert steps + 1 >= flood[pos2]
                else:
                    flood[pos2] = steps + 1
                    todo.append(pos2)
    assert len(flood) == len(nodes)
    print(max(flood.values()))

xxx = '(ESSESSENN(SSWNNWESSENN|)|N)|W'
get_distance(xxx)
get_distance('ENWWW(NEEE|SSE(EE|N))')
get_distance('ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN')
get_distance('ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))')

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

get_distance(data[1:-1])
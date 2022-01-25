#!/usr/bin/env python3

import os
import sys

import sys

sys.setrecursionlimit(1500)

from itertools import product, permutations, takewhile
from collections import defaultdict, Counter, deque

MOVES = {
    'N': (0, 1),
    'E': (1, 0),
    'S': (0, -1),
    'W': (-1, 0)
}

def tokenize(text):
    pare = 0
    part = ''
    for c in text:
        if c == '(':
            if pare == 0 and part:
                yield part
                part = ''
            part += c
            pare += 1
        elif c == ')':
            part += c
            pare -= 1
            if pare == 0:
                yield part
                part = ''
        elif c == '|' and pare == 0:
            if part:
                yield part
                part = ''
            yield c
        else:
            part += c
    if part:
        yield part

def get_tokens(text):
    if text == '':
        return [text]

    tokens = list(tokenize(text))
    if tokens[-1] == '|':
        tokens.append('')
    return tokens

class Or:
    def __init__(self, parts):
        self.parts = []

        while parts:
            chunk = list(takewhile(lambda x: x != '|', parts))
            element = parse_tokens(''.join(chunk))
            self.parts.append(element)
            parts = parts[len(chunk):]

            # ending with '|', add emptry option
            if parts == ['|']:
                self.parts.append('')
            parts = parts[1:]

    def __repr2__(self):
        return 'Or' + str(self.parts)

def parse_tokens(text):
    tokens = get_tokens(text)
    print(len(text))
    if '|' in tokens:
        return [Or(tokens)]
    elif len(tokens) == 1:
        token = tokens[0]
        if token.startswith('('):
            return parse_tokens(token[1:-1])
        else:
            return [token]
    
    result = []
    for token in tokens:
        if '(' in token or '|' in token:
            result.append(parse_tokens(token))
        else:
            result.append(token)
    return result

nodes = None
edges = None

cache = set()

def walk(pos, tokens, final=True):    
    nodes.add(pos)
    origpos = pos

    if not tokens:
        return pos

    key = (pos, str(tokens))
    if key in cache:
        return

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

    cache.add(key)
    return pos

start = (0, 0)

def get_distance(path):
    global nodes, edges

    nodes = set()
    edges = set()
    tokens = parse_tokens(path)
    print(tokens)
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


#print(parse_tokens('(A(B|)|C)|D'))

# get_distance('WNE')
get_distance('ENWWW(NEEE|SSE(EE|N))')
get_distance('ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN')
get_distance('WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))')

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

get_distance(data[1:-1])

"""
xxx = '(ESSESSENN(SSWNNWESSENN|)|N)|W'
get_distance(xxx)
get_distance('ENWWW(NEEE|SSE(EE|N))')
get_distance('ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN')
get_distance('ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))')

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()

get_distance(data[1:-1])
"""
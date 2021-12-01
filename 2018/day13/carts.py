#!/usr/bin/env python3

from termcolor import colored

route_symbols = ['|', '-', '\\', '/', '+']
cart_symbols = ['^', '>', 'v', '<']
cart_orientations = [(0, -1), (1, 0), (0, 1), (-1, 0)]
next_turns = ((3, 0, 1), (0, 1, 2), (1, 2, 3), (2, 3, 0))

data = open('input.txt').readlines()

def get_next_possible_moves(streets, position):
    c = streets[position]
    if c == '+':
        return (0, 1, 2, 3)
    elif c == '-':
        return (1, 3)
    elif c == '|':
        return (0, 2)
    elif c == '/':
        newpos = (position[0] + 1, position[1])
        if newpos in streets and (streets[newpos] == '-' or streets[newpos] == '+'):
            return (1, 2)
        else:
            return (0, 3)
    elif c == '\\':
        newpos = (position[0] + 1, position[1])
        if newpos in streets and (streets[newpos] == '-' or streets[newpos] == '+'):
            return (0, 1)
        else:
            return (2, 3)
    else:
        assert False

class Cart:
    def __init__(self, position, orientation):
        self.position = position
        self.orientation = orientation
        self.next_turn = 0

    def get_next_turns(self, streets):
        possible = get_next_possible_moves(streets, self.position)
        for i, nt in enumerate(next_turns[self.orientation]):
            if not nt in possible:
                continue
            move = cart_orientations[nt]
            newpos = (self.position[0] + move[0], self.position[1] + move[1])
            if newpos in streets:
                yield (newpos, nt, i)

    def move(self, streets):
        turns = list(self.get_next_turns(streets))
        turn = None
        if len(turns) > 1:
            assert len(turns) == 3
            turn = [x for x in turns if x[2] == self.next_turn][0]
            self.next_turn = (self.next_turn + 1) % 3
        else:
            turn = turns[0]
        self.position = turn[0]
        self.orientation = turn[1]

    def __repr__(self):
        return '%s:%s' % (str(self.position), str(self.orientation))

N = 150
def print_map(n, streets, carts):
    print('Map after: %d' % n)
    for y in range(N):
        for x in range(N):
            pos = (x, y)
            c = [c for c in carts if c.position == pos]
            if len(c):
                print(colored(cart_symbols[c[0].orientation], 'red'), end = '')
            else:
                if pos in streets:
                    print(streets[pos], end = '')
                else:
                    print(' ', end = '')
        print()

streets = {}
carts = []

for y, l in enumerate(data):
    l = l.rstrip()
    for x, c in enumerate(l):
        if c in route_symbols:
            streets[(x, y)] = c
        elif c in cart_symbols:
            index = cart_symbols.index(c)
            c = Cart((x, y), index)
            carts.append(c)
            c = '-' if c.orientation == 1 or c.orientation == 3 else '|'
            streets[(x, y)] = c

crashed = set()

def maybe_exit(carts):
    if len(carts) == 1:
        print(carts[0])
        exit(0)

for i in range(50000):
    maybe_exit(carts)
    for c in list(sorted(carts, key = lambda c: (c.position[1], c.position[0]))):
        if not c in crashed:
            c.move(streets)
            for c2 in carts:
                if c2 != c and c2.position == c.position:
                    print('Crash at %d at %s' % (i, str(c.position)))
                    crashed.add(c)
                    crashed.add(c2)
    carts = [c for c in carts if not c in crashed]

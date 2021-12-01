#!/usr/bin/env python3

data = open('input.txt').read()

p1, p2 = data.split('\n\n')

p1 = [int(x) for x in p1.splitlines()[1:]]
p2 = [int(x) for x in p2.splitlines()[1:]]

cache = {}

def totuple(p1, p2):
    return (tuple(p1), tuple(p2))

def print_score(p):
    s = 0
    for i, v in enumerate(p):
        s += (len(p) - i) * v
    print(s)

def play(p1, p2, d):
    origt = totuple(p1, p2)
    if origt in cache:
        return cache[origt]
    origt2 = totuple(p2, p1)

    print(f'Play {d} ==')
    #print(p1, p2, len(cache))

    seen = set()
    while True:
        t = totuple(p1, p2)
        if t in seen:
            return True
        seen.add(t)
        if len(p1) == 0:
            if d == 0:
                print_score(p2)
            cache[origt] = False
            return False
        if len(p2) == 0:
            if d == 0:
                print_score(p1)
            cache[origt] = True
            return True
        c1 = p1[0]
        c2 = p2[0]
        p1 = p1[1:]
        p2 = p2[1:]

        if len(p1) >= c1 and len(p2) >= c2:
            first = play(p1.copy()[:c1], p2.copy()[:c2], d+1)
        else:
            first = c1 > c2

        if first:
            p1.append(c1)
            p1.append(c2)
        else:
            p2.append(c2)
            p2.append(c1)

    assert False

play(p1, p2, 0)

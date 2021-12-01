#!/usr/bin/env python3

data = '''
....#
#..#.
#..##
..#..
#....
'''

data = '''
##.##
.#.##
##..#
#.#..
.###.
'''

moves = ((1, 0), (0, 1), (-1, 0), (0, -1))

bugs = tuple([tuple([x == '#' for x in line]) for line in data.strip().split('\n')])
seen = set()

def neighbors(d, x, y):
    if x == 2 and y == 2:
        return []
    for m in moves:
        x2 = x + m[0]
        y2 = y + m[1]
        if x2 == 2 and y2 == 2:
            continue
        if x2 >= 0 and x2 < 5 and y2 >= 0 and y2 < 5:
            yield (x2, y2, d)

all_bugs = {}
N = 210

def neighbors_multilevel(d, x, y):
    # normal
    r = list(neighbors(d, x, y))

    # outer
    if x == 0:
        r.append((1, 2, d - 1))
    elif x == 4:
        r.append((3, 2, d - 1))
    if y == 0:
        r.append((2, 1, d - 1))
    elif y == 4:
        r.append((2, 3, d - 1))

    # inner
    if x == 2 and y == 1:
        for i in range(5):
            r.append((i, 0, d + 1))
    if x == 1 and y == 2:
        for i in range(5):
            r.append((0, i, d + 1))
    if x == 3 and y == 2:
        for i in range(5):
            r.append((4, i, d + 1))
    if x == 2 and y == 3:
        for i in range(5):
            r.append((i, 4, d + 1))
    positions = list(set(r))
    n = 0
    for p in positions:
        x = p[0]
        y = p[1]
        if x == 2 and y == 2:
            assert False
        if all_bugs[p[2]][x][y]:            
            n += 1
    return n

def move(d):
    result = []
    for x in range(5):
        line = []
        for y in range(5):
            n = neighbors_multilevel(d, x, y)
            if all_bugs[d][x][y]:
                line.append(n == 1)
            else:
                line.append(n == 1 or n == 2)
        result.append(tuple(line))
    return tuple(result)

def move_multilevel():
    result = {}
    for i in range(-N, N):
        result[i] = move(i)
    for i in range(-N, N):
        all_bugs[i] = result[i]

def print_bugs(d, n):
    print('\nDepth %d after %d:' % (d, n))
    for x in range(5):
        for y in range(5):
            if x == 2 and y == 2:
                print('!' if all_bugs[d][x][y] else '?', end = '')
            else:
                print('#' if all_bugs[d][x][y] else '.', end = '')
        print()

def diversity():
    s = 0
    for x in range(5):
        for y in range(5):
            if bugs[x][y]:
                s += 2**(5 * x + y)
    return s

for i in range(-N - 1, N + 1):
    all_bugs[i] = []
    for _ in range(5):
        all_bugs[i].append(tuple([False] * 5))
    all_bugs[i] = tuple(all_bugs[i])

all_bugs[0] = bugs
print(all_bugs)

print_bugs(0, 0)

steps = 200
for i in range(steps):
    print(i)
    move_multilevel()

#for d in range(-5, 5 + 1):
#    print_bugs(d, steps)

total = 0
for d in range(-N, N):
    for x in range(5):
        for y in range(5):
            if all_bugs[d][x][y]:
                if x == 2 and y == 2:
                    assert False
                total += 1

print(total)

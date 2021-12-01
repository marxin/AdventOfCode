#!/usr/bin/env python3

from math import sqrt

asteroids = []
vapo = (26,28)

def get_vector(a1, a2):
    vector = (a2[0] - a1[0], a1[1] - a2[1])
    if vector[0] == 0:
        return (0, 1) if vector[1] > 0 else (0, -1)
    if vector[1] == 0:
        return (-1, 0) if vector[0] < 0 else (1, 0)
    a = abs(vector[1])
    return (vector[0] / a, vector[1] / a)

def get_dist(a1, a2):
    return sqrt((abs(a1[0] - a2[0]))**2 + (abs(a1[1] - a2[1])**2))

def get_visible():
    global asteroids
    global vapo
    vectores = []
    for i, a in enumerate(asteroids):
        r = get_vector(vapo, a)
        dist = get_dist(vapo, a)
        # print('%s:%s:%s' % (str(start), str(a), str(r)))
        vectores.append((r, dist))
    return vectores

lines = [x.strip() for x in open('input.txt').readlines()]

for i, line in enumerate(lines):
    for j, pixel in enumerate(line):
        if pixel == '#':
            if vapo and vapo[0] == j and vapo[1] == i:
                pass
            else:
                asteroids.append((j, i))
        elif pixel == 'X':
            vapo = (j, i)

print(vapo)
visible = get_visible()
print(visible)

print('Directions:')
print(visible)

def get_quadrant(p):
    if p[0] == 0:
        return 0 if p[1] > 0 else 4
    if p[1] == 0:
        return 2 if p[0] > 0 else 6
    if p[0] > 0 and p[1] > 0:
        return 1
    if p[0] > 0 and p[1] < 0:
        return 3
    if p[0] < 0 and p[1] < 0:
        return 5
    if p[0] < 0 and p[1] > 0:
        return 7
    assert False

def ratio(p):
    if p[1] == 0:
        return 0
    else:
        return p[0] / p[1]

print(open('input.txt').read())
print()
print(vapo)

tosort = []
for i, v in enumerate(visible):
    sorting = (get_quadrant(v[0]), ratio(v[0]), v[1]) 
    tosort.append((asteroids[i], sorting, visible[i]))

for x in tosort:
    print('%s' % str(x))

print('after sort:')
sorted = list(sorted(tosort, key = lambda x: x[1]))

for i, x in enumerate(sorted):
    print('%s' % str(x))

last_direction = None
i = 0
count = 0
while len(sorted):
    if i >= len(sorted):
        i = 0
        last_direction = None
    a = sorted[i]
    if a[1][1] != last_direction:
        count += 1
        print('Shooting #%d: %s' % (count, str(a[0])))
        sorted = sorted[:i] + sorted[i + 1:]
        last_direction = a[1][1]
    else:
        i += 1

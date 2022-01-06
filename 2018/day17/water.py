#!/usr/bin/env python3

import os
import re
import sys
from PIL import Image, ImageDraw
from collections import deque

points = set()
water = set()
sprinkle = set([(500, 0)])
surface = set()

folder = os.path.dirname(os.path.abspath(__file__))
for line in open(os.path.join(folder, 'input.txt')).readlines():
    line = line.strip()
    m = re.match('y=(.*), x=(.*)\.\.(.*)', line)
    if m:
        y = int(m.group(1))
        x1 = int(m.group(2))
        x2 = int(m.group(3))
        for x in range(x1, x2 + 1):
            points.add((x, y))
        continue
    m = re.match('x=(.*), y=(.*)\.\.(.*)', line)
    x = int(m.group(1))
    y1 = int(m.group(2))
    y2 = int(m.group(3))
    for y in range(y1, y2 + 1):
        points.add((x, y))

miny = min(p[1] for p in points)
maxy = max(p[1] for p in points)

width = 1000
height = 0

for p in points:
    if p[1] > height:
        height = p[1]

def draw():
    height = 3000
    im = Image.new('RGB', (width, height + 10), (255, 255, 255))
    draw = ImageDraw.Draw(im)

    for p in points:
        draw.point(p, fill = (0, 0, 0))
    for w in water:
        draw.point(w, fill = (0, 0, 256))
    for s in sprinkle:
        draw.point(s, fill = (0, 256, 256))
    for s in surface:
        draw.point(s, fill = (256, 0, 0))

    im.save('water.png')

def down(pos):
    return (pos[0], pos[1] + 1)

def flood_line_direction(start, direction, level):
    water.add(start)
    i = 0

    while True:
        pos = (start[0] + i * direction, start[1])
        if pos in points:
            return False
        else:
            level.add(pos)
            d = down(pos)
            if d not in water and d not in points:
                sprinkle.add(pos)
                level.remove(pos)
                return True
        i += 1


def can_remove_flood(pos):
    i = 0
    level = set()
    while True:
        i += 1
        pos2 = (pos[0] + i, pos[1])
        if pos2 in points:
            break
        elif pos2 not in water:
            return set()
        else:
            level.add(pos2)

    i = 0
    while True:
        i -= 1
        
        pos2 = (pos[0] + i, pos[1])
        if pos2 in points:
            break
        elif pos2 not in water:
            return set()
        else:
            level.add(pos2)
    return level


def flood_line(start):
    global water, surface

    level = set()
    overflow = flood_line_direction(start, 1, level)
    overflow |= flood_line_direction(start, -1, level)
    assert start in level
    water = water | level
    if overflow:
        surface = surface | level

last_zero = False
for i in range(4000):
    before = len(sprinkle) + len(water)    

    for sp in list(sprinkle):
        d = down(sp)
        if sp in water:
            continue

        if d in points or d in water and d not in surface:
            flood_line(sp)
        elif d[1] <= maxy:
            sprinkle.add(d)

    after = len(sprinkle) + len(water)
    diff = after - before
    if not diff:
        if last_zero:
            break
        last_zero = True
        for sur in list(surface):
            removed = can_remove_flood(sur)
            surface -= removed
    else:
        last_zero = False

    print(i, after - before)
        
draw()
solution = [p for p in water | sprinkle if p[1] >= miny]

print(len(solution))
print(len((water - sprinkle) - surface))

for x in sprinkle:
    water.discard(x)

for x in surface:
    water.discard(x)

sprinkle = set()
surface = set()

draw()
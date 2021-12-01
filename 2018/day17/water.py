#!/usr/bin/env python3

import re
import sys
from PIL import Image, ImageDraw
from collections import deque

points = set()
water = set()
tips = deque([(500, 0)])
sprinkle = set([(500, 0)])

for line in open('input.txt').readlines():
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

width = 1000
height = 0

for p in points:
    if p[1] > height:
        height = p[1]

def draw():
    height = 300
    im = Image.new('RGB', (width, height + 10), (255, 255, 255))
    draw = ImageDraw.Draw(im)

    for p in points:
        draw.point(p, fill = (0, 0, 0))
    for w in water:
        draw.point(w, fill = (0, 0, 256))
    for s in sprinkle:
        draw.point(s, fill = (0, 256, 256))
    im.save('water.png')

def can_flood(fall):
    if not fall in water:
        return True
    for step in [1, -1]:
        i = 1
        while True:
            p = (fall[0] + i * step, fall[1])
            if p not in water:
                break
            elif p in sprinkle:
                p2 = (p[0], p[1] - 1)
                if p2 not in sprinkle:
                    return False
            i += 1
    return True

def fill_level_up(tip):
    new_water = [tip]
    overflow = False
    for step in [1, -1]:
        i = 1
        while True:
            p = (tip[0] + i * step, tip[1])
            if p in points:
                break
            pdown = (p[0], p[1] + 1)
            if pdown in points or pdown in water:
                new_water.append(p)
            else:
                tips.append(p)
                sprinkle.add(p)
                overflow = True
                break
            i += 1
    for w in new_water:
        water.add(w)
    if not overflow:
        upper = (tip[0], tip[1] - 1)
        if upper in sprinkle:
            tips.append(upper)

stop = -1
if len(sys.argv) == 2:
    stop = int(sys.argv[1])

step = 1
while tips:
    print(step)
    step += 1
    if step == stop:
        break
    tip = tips.popleft()
    fall = (tip[0], tip[1] + 1)
    if fall in points or fall in water:
        if can_flood(fall):
            fill_level_up(tip)
    else:
        sprinkle.add(fall)
        tips.append(fall)

draw()

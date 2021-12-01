#!/usr/bin/env python3

from collections import deque

data = open('input.txt').read()

tiles = []

orig = {}

for part in data.split('\n\n'):
    lines = part.splitlines()
    id = int(lines[0].split(' ')[-1][:-1])
    lines = lines[1:]
    orig[id] = lines

    top = lines[0]
    bottom = ''.join(reversed(lines[-1]))
    left = ''.join([x[0] for x in reversed(lines)])
    right = ''.join([x[-1] for x in lines])
    tiles.append((id, deque([top, right, bottom, left])))

print(len(tiles))
d = {(0, 0): tiles[0]}

def rotate(tile):
    tile[1].rotate()

def flip(tile):
    tmp = tile[1][0][::-1]
    tile[1][0] = tile[1][2][::-1]
    tile[1][2] = tmp

    tile[1][1] = tile[1][1][::-1]
    tile[1][3] = tile[1][3][::-1]

def have_match(t0, t1, direction):
    direction2 = (direction + 2) % 4
    for j in range(2):
        if j == 1:
            flip(t1)
        for i in range(4):
            rotate(t1)
            if t0[1][direction] == t1[1][direction2][::-1]:
                return True
    return False

def place_one(tile):    
    for k, v in d.items():
        for i in range(4):
            mov = moves[i]
            pos = (k[0] + mov[0], k[1] + mov[1])
            if pos not in d:
                if have_match(v, tile, i):
                    good = True
                    for j in range(4):
                        mov2 = moves[j]
                        pos2 = (pos[0] + mov2[0], pos[1] + mov2[1])
                        if pos2 in d:
                            if not have_match(tile, d[pos2], j):
                                good = False
                    if good:
                        d[pos] = tile
                        return True
    return False


moves = [(0, -1), (1, 0), (0, 1), (-1, 0)]
tiles = tiles[1:]

while tiles:
    for tile in tiles:
        if place_one(tile):
            tiles.remove(tile)
            break

print(d.keys())

x = [p[0] for p in d.keys()]
y = [p[1] for p in d.keys()]

min_x = min(x)
max_x = max(x)
min_y = min(y)
max_y = max(y)

dx = max_x - min_x + 1
dy = max_y - min_y + 1

print(dx, dy)
print(len(d))
assert dx == dy and dx * dy == len(d)

for j in range(min_y, max_y + 1):
    for i in range(min_x, max_x + 1):
        print(d[(i, j)], end=' ')
    print()


mult = d[(min_x, min_y)][0] * d[(max_x, min_y)][0] * d[(min_x, max_y)][0] * d[(max_x, max_y)][0]
print(mult)
print()

def rotate_image(image):
    H = len(image)
    W = len(image[0])
    image2 = []
    for i in range(W):
        line = ''
        for j in range(H):
            line += image[H - j - 1][i]
        image2.append(line)
    return image2

def flip_image(image):
    return [line[::-1] for line in image]

def find_position(image, top_line, bottom_line):
    for flip in range(2):
        if flip == 1:
            image = flip_image(image)
        for i in range(4):
            image = rotate_image(image)
            if image[0] == top_line:
                assert image[-1] == bottom_line[::-1]
                return image
    assert False

left_top = d[(min_x, min_y)]
print(left_top)
print()

output = []

def crop_to_8(image):
    lines = image[1:-1]
    return [l[1:-1] for l in lines]

output = [''] * (max_x - min_x + 1) * 8

print(min_x, min_y, max_x, max_y)

for i in range(min_x, max_x + 1):
    for j in range(min_y, max_y + 1):
        tile = d[(i, j)]
        image = orig[tile[0]]
        updated = crop_to_8(find_position(image, tile[1][0], tile[1][2]))
        for k, line in enumerate(updated):
            output[8 * (j - min_y) + k] += line
        # print('\n'.join(updated))
        # print('\n'.join(output))

"""
image = orig[3001]
print('\n'.join(image))
print()
print('\n'.join(find_position(image, left_top[1][0], left_top[1][2])))

print('\n'.join(output))
print()
"""

monster = ['..................#.', '#....##....##....###', '.#..#..#..#..#..#...']

def find_monster_from(monster, y, x, mark):
    for i in range(len(monster)):
        for j in range(len(monster[0])):
            v = monster[i][j]
            y2 = i + y
            x2 = j + x
            if y2 >= len(output) or x2 >= len(output[0]):
                return False
            if v == '#' and y2 < len(output) and x2 < len(output[0]):
                o = output[y2][x2]
                if o == '.':
                    return False
                elif mark:
                    assert o == '#'
                    output[y2] = output[y2][:x2] + '█' + output[y2][x2 + 1:]
    return True

def find_monster(monster):
    mcount = 0
    for i in range(len(output)):
        for j in range(len(output[0])):
            if find_monster_from(monster, i, j, False):
                assert find_monster_from(monster, i, j, True) == True
                mcount += 1
    return mcount

print(''.join(output).count('#'))
mcount = find_monster(flip_image(monster))

print('\n'.join((monster)))
print()
print('\n'.join(output))

#print()
#print('\n'.join(rotate_image(monster)))

print(mcount, ''.join(output).count('#'))

print(len(output))
print(len(output[0]))

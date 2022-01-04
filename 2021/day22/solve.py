#!/usr/bin/env python3

import os

DIM = 3


class Box:
    def __init__(self, start, end, on=None):
        self.start = start
        self.end = end        
        self.end_minus_one = self._end_minus_one()
        self.on = on

        for d in range(DIM):
            assert self.start[d] < self.end[d]

    @classmethod
    def parse_line(cls, line):
        op, coords = line.split()
        coords = [[int(y) for y in x[2:].split('..')] for x in coords.split(',')]
        start = [x[0] for x in coords]
        end = [x[1] + 1 for x in coords]
        return Box(start, end, op == 'on')

    @classmethod
    def get_subcubes(cls, cubes):
        points = []
        for c in cubes:
            points += [c.start, c.end]

        x_values = sorted(set(p[0] for p in points))
        y_values = sorted(set(p[1] for p in points))
        z_values = sorted(set(p[2] for p in points))

        result = []
        for x in range(len(x_values) - 1):
            for y in range(len(y_values) - 1):
                for z in range(len(z_values) - 1):
                    split = Box((x_values[x], y_values[y], z_values[z]),
                                (x_values[x + 1], y_values[y + 1],
                                z_values[z + 1]))
                    result.append(split)
        return result

    def _end_minus_one(self):
        return tuple([p - 1 for p in self.end])

    def volume(self):
        p = 1
        for d in range(DIM):
            p *= self.end[d] - self.start[d]
        return p

    def is_point_in(self, point):
        for d in range(DIM):
            if not (self.start[d] <= point[d] < self.end[d]):
                return False
        return True

    def points_in(self, other):
        inter = 0
        if other.is_point_in(self.start):
            inter += 1
        if other.is_point_in(self.end_minus_one):
            inter += 1
        return inter

    def intersects(self, other):
        for d in range(DIM):
            if other.end[d] < self.start[d] or self.end[d] < other.start[d]:
                return False
        return True

    def __repr__(self):
        return f'Box[{self.start}:{self.end})'


folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

b = Box([0, 0, 0], [1, 2, 3])
assert b.volume() == 1 * 2 * 3

data = set()
actions = []
enabled = []

for i, line in enumerate(lines):
    actions.append(Box.parse_line(line))

print('Actions:', len(actions))

for i, action in enumerate(actions):
    print(action)
    print(i + 1, '/', len(actions), len(enabled))

    if enabled:
        enabled2 = []
        for enable in enabled:
            if enable.points_in(action) == 2:
                assert action.points_in(enable) == 0
            elif not enable.intersects(action):
                enabled2.append(enable)
            else:
                for split in Box.get_subcubes([action, enable]):
                    points_in_action = split.points_in(action)
                    points_in_enable = split.points_in(enable)
                    if points_in_action == 2:
                        pass
                    elif points_in_enable == 2:
                        assert points_in_action == 0
                        enabled2.append(split)
                    else:
                        assert points_in_action == 0 and points_in_enable == 0

        enabled = enabled2
    if action.on:
        enabled.append(action)

solution = sum([x.volume() for x in enabled])
print(solution)
assert solution == 1322825263376414
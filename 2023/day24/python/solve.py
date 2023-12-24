import os
from z3 import *

s = Solver()

lines = open('input.txt').read().splitlines()
TS = [Int(f'T{i}') for i in range(0, len(lines))]
xx0, yy0, zz0, xx, yy, zz = Int('x0'), Int('y0'), Int('z0'), Int('x'), Int('y'), Int('z')

for i, l in enumerate(lines):
    parts = [int(x) for x in l.replace('@', ',').split(',')]
    x0, y0, z0, x, y, z = parts
    s.add(x0 + TS[i] * x == xx0 + TS[i] * xx)
    s.add(y0 + TS[i] * y == yy0 + TS[i] * yy)
    s.add(z0 + TS[i] * z == zz0 + TS[i] * zz)

s.check()
m = s.model()
print(eval(str(m[xx0] + m[yy0] + m[zz0])))

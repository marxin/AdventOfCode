#!/usr/bin/env python3

def get_level(x, y, sn):
    rid = x + 10
    v = (rid * y + sn) * rid
    digit = int(v / 100) % 10
    return digit - 5

N = 300
SN = 5235
d = {}

for i in range(N):
    for j in range(N):
        d[(i, j)] = get_level(i, j, SN)

maximum = 0
coo = None
for size in range(1, 30):
    print('size: %d' % size)
    for i in range(N - size):
        for j in range(N - size):
            s = 0
            for x in range(size):
                for y in range(size):
                    s += d[(i + x, j + y)]
            if s > maximum:
                maximum = s
                coo = (i, j)
                print('%d:%s' % (maximum, str(coo)))

print(maximum)
print(coo)

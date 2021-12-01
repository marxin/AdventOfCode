#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

def paper(l, w, h):
    a = l*w
    b = w*h
    c = h*l
    return 2 * (a + b + c) + min(a, b, c)

s = 0
s2 = 0
for l in lines:
    parts = [int(x) for x in l.split('x')]
    parts = sorted(parts)
    l, w, h = parts
    s += paper(l, w, h)
    s2 += 2 *  (parts[0] + parts[1]) + l * w * h

print(s)
print(s2)

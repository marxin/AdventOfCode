#!/usr/bin/env python3

from textwrap import wrap

data = open('input.txt').read().strip()
width = 25
height = 6
n = width * height

layers = wrap(data, n)

result = ''
for i in range(n):
    for l in layers:
        pixel = l[i]
        if pixel != '2':
            result += pixel
            break


for i in wrap(result, width):    
    print(i.replace('0', ' ').replace('1', 'X'))

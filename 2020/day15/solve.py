#!/usr/bin/env python3

d = {}
numbers = [0, 3, 6]
numbers = [9,12,1,4,17,0,18]

for i, n in enumerate(numbers[:-1]):
    d[n] = i + 1

last = numbers[-1]

for i in range(len(d) + 1, 30000000):
    if i % 100000 == 0:
        print(i)
    # print(d, i, last)
    previous = last
    if last in d:
        last = i - d[last]
    else:
        last = 0

    d[previous] = i

print(last)

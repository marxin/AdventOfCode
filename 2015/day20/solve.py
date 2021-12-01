#!/usr/bin/env python3

minimum = 29000000

N = 2000000
a = [0] * N

for i in range(1, N):
    for j in range(1, 51):
        index = i * j
        if index < N:
            a[index] += 11 * i

print(minimum)
print(a[N - 1])

for i in range(1, N):
    if a[i] >= minimum:
        print(i)
        break

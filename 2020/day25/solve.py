#!/usr/bin/env python3

n = 1
sb = 7

def transform(public_key):
    n = 1
    i = 0
    while n != public_key:
        n *= sb
        n %= 20201227
        i += 1
    return i

def transform2(n, loop_size):
    k = 1
    for i in range(loop_size):
        k *= n
        k %= 20201227
    return k

a = 1614360
b = 7734663

#a = 5764801
#b = 17807724

loop1 = transform(a)
loop2 = transform(b)

print(loop1, loop2)

print(transform2(a, loop2))
print(transform2(b, loop1))

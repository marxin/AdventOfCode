#!/usr/bin/env python3

import hashlib

data = open('input.txt').read().rstrip()

for i in range(1, 10000000):
    h = hashlib.md5()
    h.update((data + str(i)).encode())
    digest = h.hexdigest()
    if digest.startswith('000000'):
        print(i)
        break

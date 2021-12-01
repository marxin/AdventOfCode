#!/usr/bin/env python3

def bisect(string, value=0):
    if string:
        c = string[0]
        if c == 'F' or c == 'L':
            pass
        elif c == 'B' or c == 'R':
            value += 2 ** (len(string) - 1)
        else:
            assert False
        return bisect(string[1:], value)
    else:
        return value

def getid(string):
    return bisect(string[:7]) * 8 + bisect(string[7:])

# print(getid('FBFBBFFRLR'))
# print(getid('BBFFBBFRLL'))

maximum = 0
ids = []
for line in open('input.txt').read().splitlines():
    id = getid(line)
    ids.append(id)
    if id > maximum:
        maximum = id
print(maximum)
print()

ids = sorted(ids)
for i, v in enumerate(ids):
    if i > 0 and ids[i - 1] + 1 != v:
        print(v - 1)
        break

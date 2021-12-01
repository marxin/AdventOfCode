#!/usr/bin/env python3

know = {
'children': 3,
'cats': 7,
'samoyeds': 2,
'pomeranians': 3,
'akitas': 0,
'vizslas': 0,
'goldfish': 5,
'trees': 3,
'cars': 2,
'perfumes': 1
}

print(know)

data = open('input.txt').read()
lines = data.splitlines()

for index, line in enumerate(lines):
    i = line.find(': ') + 2
    parts = line[i:].split(',')
    d = {}
    for p in parts:
        p = p.strip()
        k, v = p.split(':')
        d[k] = int(v)

    allok = True
    for k, v in know.items():
        if k in d and v != d[k]:
            allok = False
            break
    if allok:
        print(d)
        print(index + 1)

    allok = True
    for k, v in know.items():
        if k in d:
            if k == 'cats' or k == 'trees':
                if d[k] <= v:
                    allok = False
                    break
            elif k == 'pomeranians' or k == 'goldfish':
                if d[k] >= v:
                    allok = False
                    break
            elif d[k] != v:
                allok = False
                break

    if allok:
        print(d)
        print(index + 1)

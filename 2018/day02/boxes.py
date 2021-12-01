#!/usr/bin/env python3

def has_arity(text, n):
    d = {}
    for s in text:
        if not s in d:
            d[s] = 1
        else:
            d[s] += 1
    for k, v in d.items():
        if v == n:
            return True
    return False

boxes = [l.strip() for l in open('input.txt').readlines()]

two = sum([int(has_arity(x, 2)) for x in boxes])
three = sum([int(has_arity(x, 3)) for x in boxes])

print(two)
print(three)
print(two * three)

def get_difference(text1, text2):
    r = []
    for i, v in enumerate(text1):
        r.append(ord(v) - ord(text2[i]))
    return r

def are_similar(text1, text2):
    diffs = 0
    for i, v in enumerate(text1):
        if v != text2[i]:
            diffs += 1
            if diffs == 2:
                return False
    return True

for t1 in boxes:
    for t2 in boxes:
        if t1 != t2 and are_similar(t1, t2):
            print(t1)
            print(t2)
            print(get_difference(t1, t2))

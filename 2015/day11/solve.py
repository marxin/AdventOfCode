#!/usr/bin/env python3

start = 'hepxcrrq'
start = 'hepxxyzz'

def is_good(pw):
    if 'i' in pw or 'o' in pw or 'l' in pw:
        return False

    have = False
    for i in range(len(pw) - 2):
        a = ord(pw[i])
        b = ord(pw[i + 1])
        c = ord(pw[i + 2])
        if a + 1 == b and b + 1 == c:
            have = True
            break
    if not have:
        return False

    pairs = 0
    i = 0
    while i < len(pw) - 1:
        if pw[i] == pw[i + 1]:
            pairs += 1
            i += 1
        i += 1
    return pairs >= 2

def nextpw(pw):
    chars = list(reversed(pw))
    i = 0
    while chars[i] == 'z':
        chars[i] = 'a'
        i += 1
    chars[i] = chr(ord(chars[i]) + 1)
    return ''.join(reversed(chars))

assert is_good('abcdffaa')
assert not is_good('hijklmmn')

start = nextpw(start)
while True:
    if is_good(start):
        print(start)
        break
    else:
        start = nextpw(start)

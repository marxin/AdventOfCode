#!/usr/bin/env python3

def reduce_once(text):
    r = []
    i = 0
    while i < len(text):
        c = text[i]
        if i < len(text) - 1:
            cn = text[i + 1]
            if c.lower() == cn.lower() and c != cn:
                i += 2
                continue
        r.append(c)
        i += 1
    return ''.join(r)

input = open('input.txt').read().strip()

print(len(input))

def get_length(input, letter):
    input = input.replace(letter, '').replace(letter.upper(), '')
    while True:
        reduced = reduce_once(input)
        if reduced == input:
            break
        else:
            input = reduced
    return len(input)

minimum = 2**20
for i in range(26):
    letter = chr(ord('a') + i)
    length = get_length(input, letter)
    if length < minimum:
        minimum = length
        print('New minimum: ', end = '')
    print('%d: %s' % (length, letter))


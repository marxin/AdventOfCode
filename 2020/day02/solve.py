#!/usr/bin/env python3

count = 0
count2 = 0

def valid(range, letter, password):
    c = password.count(letter)
    return range[0] <= c and c <= range[1]

def valid_new(range, letter, password):
    v1 = password[range[0] - 1] == letter
    v2 = password[range[1] - 1] == letter
    return (v1 and not v2) or (not v1 and v2)

for line in open('input.txt').read().splitlines():
    parts = line.split(' ')
    range = [int(x) for x in parts[0].split('-')]
    letter = parts[1].rstrip(':')
    password = parts[2]
    
    if valid(range, letter, password):
        count += 1
    if valid_new(range, letter, password):
        count2 += 1

print(count)
print(count2)

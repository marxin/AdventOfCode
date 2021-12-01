#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

def is_nice(string):
    for x in ['ab', 'cd', 'pq', 'xy']:
        if x in string:
            return False
    if len([x for x in string if x in 'aeiou']) < 3:
        return False
    if all(string[i] != string[i + 1] for i in range(len(string) - 1)):
        return False
    return True 

def has_double(s):
    for i in range(len(s) - 2):
        needle = s[i:i + 2]
        if s.count(needle) >= 2:
            return True
    return False

def in_between(s):
    for i in range(len(s) - 2):
        if s[i] == s[i + 2]:
            return True
    return False

def is_nice2(s):
    return has_double(s) and in_between(s)

print(len([l for l in lines if is_nice(l)]))
print(len([l for l in lines if is_nice2(l)]))

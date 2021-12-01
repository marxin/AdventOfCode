#!/usr/bin/env python3

import re


MANDATORY = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}

passparts = []
buffer = ''
for line in open('input.txt').readlines():
    line = line.strip()
    if line:
        buffer += ' ' + line
    else:
        passparts.append(buffer)
        buffer = ''

if buffer:
    passparts.append(buffer)

parsed_passwords = []
for p in passparts:
    d = {}
    for part in p.split(' '):
        if part:
            part2 = part.split(':')        
            d[part2[0]] = part2[1]
    parsed_passwords.append(d)

print(len([p for p in parsed_passwords if not MANDATORY - set(p.keys())]))

def is_valid(passport):
    year = int(passport['byr'])
    if not (year >= 1920 and year <= 2002):
        return False
    year = int(passport['iyr'])
    if not (year >= 2010 and year <= 2020):
        return False
    year = int(passport['eyr'])
    if not (year >= 2020 and year <= 2030):
        return False
    hgt = passport['hgt']
    if hgt.endswith('cm'):
        hgt = int(hgt[:-2])
        if not (hgt >= 150 and hgt <= 193):
            return False
    elif hgt.endswith('in'):
        hgt = int(hgt[:-2])
        if not (hgt >= 59 and hgt <= 76):
            return False
    else:
        return False
    color = passport['hcl']
    if not re.match('#[0-9a-f]{6}$', color):
        return False
    if not passport['ecl'] in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}:
        return False
    if not re.match('\d{9}$', passport['pid']):
        return False

    return True
        

print(len([p for p in parsed_passwords if not MANDATORY - set(p.keys()) and is_valid(p)]))

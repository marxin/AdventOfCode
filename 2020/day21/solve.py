#!/usr/bin/env python3

import sys

data = open('input.txt').read()
lines = data.splitlines()

PREFIX='contains '

rules = []
alergens = set()

for line in lines:
    lhs, rhs = line.split(' (')
    rhs = rhs.rstrip(')')[len(PREFIX):].split(', ')
    assert len(rhs) == len(set(rhs))
    rhs = set(rhs)
    alergens.update(rhs)
    lhs = lhs.split(' ')
    assert len(lhs) == len(set(lhs))
    lhs = set(lhs)
    rules.append((lhs, rhs))

print(alergens)
print(len(alergens))

rules = sorted(rules, key=lambda x: (len(x[1]), len(x[0])))

def check(mapping):
    for lhs, rhs in rules:
        for r in rhs:
            if r in mapping and not mapping[r] in lhs:
                return False
    return True

def report(mapping):
    total = 0
    for lhs, rhs in rules:
        total += len(lhs - set(mapping.values()))
    print(total)
    s = sorted(mapping.items(), key=lambda x:x[0])
    print(s)
    print(','.join([x[1] for x in s]))

def solve(mapping, i):
    print(len(mapping), mapping)
    if len(mapping) == len(alergens):
        report(mapping)
        sys.exit(0)
    rule = rules[i]
    unknown = rule[1] - set(mapping.keys())
    if unknown:
        for u in unknown:
            available_lhs = [x for x in rule[0] if x not in mapping.values()]
            for a in available_lhs:
                mapping[u] = a
                if check(mapping):
                    solve(mapping, i + 1)
                del mapping[u]
    else:
        solve(mapping, i + 1)


print(rules[0])
solve({}, 0)

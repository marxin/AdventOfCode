#!/usr/bin/env python3

data = open('input.txt').read()
numbers = set([int(x) for x in data.splitlines()])

#numbers = set([1, 3, 4, 5, 7, 2, 8, 10, 11, 9])

s = sum(numbers)
part = int(s / 4)
min_count = 10**10
quantum = 10**20

cache = set()

def arange(bag, available):
    global quantum
    global cache
    global min_count
    if frozenset(bag) in cache:
        return
    l = len(bag)
    if l > min_count:
        cache.add(frozenset(bag))
        return
    if sum(bag) == part:
        cache.add(frozenset(bag))
        if l <= min_count:
            min_count = l
            x = 1
            for b in bag:
                x *= b
            if x < quantum:
                quantum = x
                print(sum(bag), bag, x)
    elif sum(bag) > part:
        cache.add(frozenset(bag))
        return
    for i in reversed(list(available)):
        bag.add(i)
        available.remove(i)
        arange(bag, available)
        cache.add(frozenset(bag))
        available.add(i)
        bag.remove(i)

arange(set(), numbers)

print(quantum)

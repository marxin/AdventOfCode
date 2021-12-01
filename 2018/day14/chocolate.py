#!/usr/bin/env python3

recipes = [3, 7]

a = 0
b = 1

steps = list(reversed([3,3,0,1,2,1]))

for i in range(10**9):
    if i % 10**6 == 0:
        print(i)
    if len(recipes) >= len(steps):
        seen = True
        for i, v in enumerate(steps):
            if v != recipes[len(recipes) - i - 2]:
                seen = False
                break
        if seen:
            print(len(recipes) - len(steps))
            exit(0)

    s = recipes[a] + recipes[b]
    if s >= 10:
        n1 = s % 10
        n0 = int(s / 10)
        recipes.append(n0)
        recipes.append(n1)
    else:
        recipes.append(s)
    a = (a + recipes[a] + 1) % len(recipes)
    b = (b + recipes[b] + 1) % len(recipes)

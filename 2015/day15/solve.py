#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

recipes = []
N = 100

for line in lines:
    recipe = {}
    parts = line.split(':')[1].split(',')
    for p in parts:
        key, value = p.strip().split(' ')
        recipe[key] = int(value)
    recipes.append(recipe)

best = 0
def mix(values):
    global best
    suma = 1
    for k in recipes[0].keys():
        total = 0
        for i, value in enumerate(values):
            total += value * recipes[i][k]
        if k == 'calories':
            if total == 500:
                continue
            else:
                return
        if total <= 0:
            return
        suma *= total
    if suma > best:
        print(values)
        best = suma

def generate(values, N, S, TOTAL):
    if S > TOTAL:
        return

    if len(values) == N:
        if sum(values) == TOTAL:
            mix(values)
        return

    for i in range(1, TOTAL):
        values.append(i)
        generate(values, N, S + i, TOTAL)
        values.pop()

generate([], len(recipes), 0, 100)
print(best)

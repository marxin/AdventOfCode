#!/usr/bin/env python3

rules_input = '''
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50
'''

rules_input = '''
departure location: 29-917 or 943-952
departure station: 50-875 or 884-954
departure platform: 41-493 or 503-949
departure track: 50-867 or 875-966
departure date: 30-655 or 679-956
departure time: 46-147 or 153-958
arrival location: 50-329 or 344-968
arrival station: 42-614 or 623-949
arrival platform: 35-849 or 860-973
arrival track: 42-202 or 214-959
class: 38-317 or 329-968
duration: 44-530 or 539-953
price: 28-713 or 727-957
route: 30-157 or 179-966
row: 38-114 or 136-969
seat: 45-441 or 465-956
train: 44-799 or 824-951
type: 41-411 or 437-953
wagon: 39-79 or 86-969
zone: 48-306 or 317-974
'''

myticket = [191,89,73,139,71,103,109,53,97,179,59,67,79,101,113,157,61,107,181,137]

d = {}
rules = set()
for line in rules_input.strip().splitlines():
    parts = line.split(': ')
    name = parts[0]
    d[name] = set()
    ranges = parts[1].strip()
    for rule in ranges.split(' or '):
        start, end = [int(x) for x in rule.split('-')]
        for i in range(start, end + 1):
            rules.add(i)
            d[name].add(i)

valid = []

suma = 0
for line in open('input.txt').read().splitlines():
    numbers = [int(x) for x in line.split(',')]
    invalid = [n for n in numbers if n not in rules]
    suma += sum(invalid)
    if not invalid:
        valid.append(numbers)
        # print(numbers)

print(suma)
print(f'valid: {len(valid)}')

valid_indices = {}

for key in d:
    valid_indices[key] = set()
    for i in range(len(d)):
        if all([v[i] in d[key] for v in valid]):
            valid_indices[key].add(i)

mapping = {}

while len(mapping) != len(d):
    for key in valid_indices:
        values = list(valid_indices[key])
        if len(values) == 1:
            index = values[0]
            mapping[key] = index
            for key, value in valid_indices.items():
                value.discard(index)
            break

print(mapping)
mult = 1

for k, v in mapping.items():
    if 'departure' in k:
        mult *= myticket[v]

print(mult)

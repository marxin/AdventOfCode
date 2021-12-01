#!/usr/bin/env python3

numbers = [int(l) for l in open('input.txt').read().splitlines()]
N = 25

def is_sum(number, start):
    for i in range(start, start + N):
        for j in range(start, start + N):
            if i != j and numbers[i] + numbers[j] == number:
                return True

i = N
while True:
    if not is_sum(numbers[i], i - N):
        target = numbers[i]
        break
    i += 1

print(target)
print(len(numbers))

for i in range(len(numbers)):
    subset = [numbers[i]]
    while sum(subset) <= target:
        if sum(subset) == target:
            print(subset)
            subset = sorted(subset)
            print(f'RESULT: {subset[0] + subset[-1]}')
            break
        else:
            i += 1
            subset.append(numbers[i])

# [865846, 920316, 975234, 934803, 1683508, 961814, 1072768, 1291799, 1108823, 1295819, 1461710, 1669021, 1467432, 1472332, 3145218, 1475958, 1476524]

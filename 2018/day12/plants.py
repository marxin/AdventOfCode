#!/usr/bin/env python3

initial = '..##.#######...##.###...#..#.#.#..#.##.#.##....####..........#..#.######..####.#.#..###.##..##..#..#'

patterns = '''
#..#. => .
..#.. => .
..#.# => #
##.#. => .
.#... => #
#.... => .
##### => #
.#.## => .
#.#.. => .
#.### => #
.##.. => #
##... => .
#...# => #
####. => #
#.#.# => .
#..## => .
.#### => .
...## => .
..### => #
.#..# => .
##..# => #
.#.#. => .
..##. => .
###.. => .
###.# => #
#.##. => #
..... => .
.##.# => #
....# => .
##.## => #
...#. => #
.###. => .
'''

initial2 = '#..#.#..##......###...###'

patterns2 = '''
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
'''

d = {}
for p in patterns.strip().split('\n'):
    p = p.strip()
    d[p[:5]] = p[-1]

N = 2000
D = 50
N2 = N + D

initial = '.' * N2 + initial + '.' * N2

print(initial)
for i in range(N):
    s = '..'
    for x in range(len(initial) - 2):
        chunk = initial[x:x + 5]
        if chunk in d:
            s += d[chunk]
        else:
            s += '.'
    # print(s)
    initial = s

print(len([x for x in s if x == '#']))

s = 0
indices = []
for i in range(len(initial)):
    if initial[i] == '#':
        x = i - N2
        indices.append(x)

print(indices)
print(sum(indices))

indices2 =[x - N for x in indices]
print(indices2)

BIG = 50000000000
s = 0
for i in indices2:
    s += (BIG + i)

print(s)

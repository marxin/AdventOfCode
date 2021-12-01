#!/usr/bin/env python3

from datetime import datetime

lines = open('input.txt').readlines()

records = []

for l in lines:
    l = l.strip()
    i = l.index(']')
    date = l[1:i]
    d = datetime.strptime(date, '%Y-%m-%d %H:%M')
    records.append((d, l[i + 2:]))

guards = {}

last_guard = None
last_sleep_minute = None

for rec in sorted(records, key = lambda x: x[0]):
    parts = rec[1].split(' ')
    minute = rec[0].minute
    if parts[0] == 'Guard':
        id = int(parts[1][1:])
        last_guard = id
        last_sleep_minute = None
    elif parts[0] == 'falls':
        last_sleep_minute = minute
        assert rec[0].hour == 0
    elif parts[0] == 'wakes':
        assert rec[0].hour == 0
        if not last_guard in guards:
            guards[last_guard] = [0] * 60
        for i in range(last_sleep_minute, minute):
            guards[last_guard][i] += 1 
    else:
        assert False

print(guards)

maximum = 0
maximum_id = 0
maximum_num_sleeps = [0, 0, 0]

for id, sleeping in guards.items():
    sleeps = sum(sleeping)
    max_sleep = max(sleeping)
    if max_sleep > maximum_num_sleeps[0]:
        maximum_num_sleeps[0] = max_sleep
        maximum_num_sleeps[1] = id
        maximum_num_sleeps[2] = sleeping.index(max_sleep)
    if sleeps > maximum:
        maximum = sleeps
        maximum_id = id

print(maximum_id)

maximum = 0
maximum_minute = None
for i in range(60):
    if guards[maximum_id][i] > maximum:
        maximum = guards[maximum_id][i]
        maximum_minute = i

print(maximum_minute)
print(maximum_id * maximum_minute)

print(maximum_num_sleeps)
print(maximum_num_sleeps[1] * maximum_num_sleeps[2])

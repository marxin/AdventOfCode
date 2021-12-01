#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

N = 2503

reindeers = []
for line in lines:
    parts = line.split(' ')
    speed = int(parts[3])
    duration = int(parts[6])
    sleep = int(parts[-2])
    reindeers.append((speed, duration, sleep))

def move(reindeer):
    t = 0
    distance = 0
    duration = reindeer[1]
    tosleep = 0
    d = {}

    while t <= N:
        if t >= 1:
            d[t] = distance

        if tosleep > 0:
            tosleep -= 1
            t += 1
            continue
        elif tosleep == 0:
            duration = reindeer[1]
            tosleep = -1

        if duration == 0:
            tosleep = reindeer[2] - 1
        else:
            distance += reindeer[0]
            duration -= 1
        t += 1
    return (distance, d)

print(reindeers)

distances = [move(r) for r in reindeers]
maxdist = max([d[0] for d in distances])
print(maxdist)
assert maxdist == 2655

scores = [d[1] for d in distances]

score_card = [0] * len(distances)

for t in range(N):
    if t == 0:
        continue
    best = 0
    besti = []
    for i, score in enumerate(scores):
        s = score[t]
        if s > best:
            best = s
            besti = [i]
        elif s == best:
            besti.append(i)

    for bi in besti:
        score_card[bi] += 1

print(score_card)
print(max(score_card))

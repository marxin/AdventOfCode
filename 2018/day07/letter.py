#!/usr/bin/env python3

lines = open('input.txt').readlines()

delta = 60
N = 5
dependencies = {}
workers = [None] * N

for l in lines:
    parts = l.split(' ')
    needed = parts[1]
    node = parts[-3]
    if not node in dependencies:
        dependencies[node] = set()
    dependencies[node].add(needed)
    if not needed in dependencies:
        dependencies[needed] = set()

def get_available_letters():
    if not len(dependencies):
        return None
    candidates = [k for k, v in dependencies.items() if not len(v)]
    return list(sorted(candidates))

def letter_done(letter):
    del dependencies[letter]
    for k, v in dependencies.items():
        v.discard(letter)

doing = set()
print(dependencies)
for t in range(1100):
    for w in range(N):
        if workers[w] and workers[w][1] == t:
            # job is done
            print('Worker #%d finished %s at %d' % (w, workers[w][0], t))
            letter_done(workers[w][0])
            doing.remove(workers[w][0])
            workers[w] = None

    available = [x for x in get_available_letters() if not x[0] in doing]
    for w in range(N):
        if workers[w] == None:
            # assign the job to worker
            if len(available):
                job = available[0]
                duration = delta + ord(job) - ord('A') + 1 + t
                print('Worker #%d starting %s at %d (will finish at %d)' % (w, job, t, duration))
                workers[w] = (job, duration)
                doing.add(job)
                available = available[1:]
            else:
                break

print(dependencies)

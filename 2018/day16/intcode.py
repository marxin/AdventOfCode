#!/usr/bin/env python3

opcodes = {
        'addr': lambda regs, op: regs[op[1]] + regs[op[2]],
        'addi': lambda regs, op: regs[op[1]] + op[2],
        'mullr': lambda regs, op: regs[op[1]] * regs[op[2]],
        'mulli': lambda regs, op: regs[op[1]] * op[2],
        'banr': lambda regs, op: regs[op[1]] & regs[op[2]],
        'bani': lambda regs, op: regs[op[1]] & op[2],
        'borr': lambda regs, op: regs[op[1]] | regs[op[2]],
        'bori': lambda regs, op: regs[op[1]] | op[2],
        'setr': lambda regs, op: regs[op[1]],
        'seti': lambda regs, op: op[1],
        'gtir': lambda regs, op: 1 if op[1] > regs[op[2]] else 0,
        'gtri': lambda regs, op: 1 if regs[op[1]] > op[2] else 0,
        'gtrr': lambda regs, op: 1 if regs[op[1]] > regs[op[2]] else 0,
        'eqir': lambda regs, op: 1 if op[1] == regs[op[2]] else 0,
        'eqri': lambda regs, op: 1 if regs[op[1]] == op[2] else 0,
        'eqrr': lambda regs, op: 1 if regs[op[1]] == regs[op[2]] else 0
        }

possible_opcodes = {}

def execute(lines):
    before = eval(lines[0][len('Before: '):])
    after = eval(lines[2][len('After: '):])
    ops = [int(x) for x in lines[1].split(' ')]
    valid = 0
    for code, fn in opcodes.items():
        regs = before.copy()
        regs[ops[3]] = fn(regs, ops)
        if regs == after:
            valid += 1
            if not ops[0] in possible_opcodes:
                possible_opcodes[ops[0]] = set()
            possible_opcodes[ops[0]].add(code)
    return valid

lines = open('input.txt').readlines()

good = 0
all = 0
while len(lines):
    r = execute(lines[:3])
    if r >= 3:
        good += 1
    lines = lines[4:]
    all += 1

print(all)
print(good)

for k, v in sorted(possible_opcodes.items(), key = lambda x: x[0]):
    print('%s:%s' % (k, str(sorted(list(v)))))

known = {}

while possible_opcodes:
    single = [(k, v) for k, v in possible_opcodes.items() if len(v) == 1]
    assert len(single) > 0
    intcode, opcode = single[0]
    opcode = list(opcode)[0]
    known[intcode] = opcode
    del possible_opcodes[intcode]
    for k in possible_opcodes.keys():
        possible_opcodes[k].discard(opcode)

print(known)
regs = [0] * 4

for line in open('input2.txt').readlines():
    ops = [int(x) for x in line.split(' ')]
    fn = opcodes[known[ops[0]]]
    print(regs)
    regs[ops[3]] = fn(regs, ops)

print(regs)

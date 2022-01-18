#!/usr/bin/env python3

import os
import sys

regs = [0] * 6

opcodes = {
        'addr': lambda regs, op: regs[op[1]] + regs[op[2]],
        'addi': lambda regs, op: regs[op[1]] + op[2],
        'mulr': lambda regs, op: regs[op[1]] * regs[op[2]],
        'muli': lambda regs, op: regs[op[1]] * op[2],
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

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()
ip = int(lines[0].split()[-1])

instructions = lines[1:]

# part 2
regs[0] = 1

counter = 0

hack1 = 10551288

progress = []
last0 = 0

while regs[ip] < len(instructions):
    counter += 1
    print(counter, regs)

    # if regs == [0, 2, 9, 10551288, 1, 0]:
        #regs[3] = 1000

    

    """
    if regs == [1, 10, 9, 10551288, 100, 0]:
        regs[4] = hack1 - 1
    elif regs[1] == 10 and regs[2] == 9:
        regs[1] = hack1 - 1
    elif regs == [1, 100, 5, 10551288, 10551287, 0]:
        regs[1] = hack1 - 1
    """
    
    if counter > 1000:
        if regs[0] != last0:
            print(regs[0] - last0)
            last0 = regs[0]

    if regs == [1, 10, 9, 10551288, 100, 0]:
        regs[4] = hack1 - 1

    parts = instructions[regs[ip]].split()
    ops = [None] + [int(x) for x in parts[1:]]
    regs[ops[3]] = opcodes[parts[0]](regs, ops)
    regs[ip] += 1

print(regs[0])
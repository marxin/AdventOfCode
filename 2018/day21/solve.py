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

regs[0] = 0

counter = 0

while regs[ip] < len(instructions):
    counter += 1
    insn = instructions[regs[ip]]
    parts = insn.split()
    ops = [None] + [int(x) for x in parts[1:]]
    before = regs.copy()
    regs[ops[3]] = opcodes[parts[0]](regs, ops)
    regs[ip] += 1
    print(f'ip(#{ip})={counter}', before, '->', regs, '//', insn)

print(regs[0])
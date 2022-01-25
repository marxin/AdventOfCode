#!/usr/bin/env python3

import os
import sys


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

def run(value, limit, verbose):
    regs = [0] * 6
    regs[0] = value
    counter = 0

    while regs[ip] < len(instructions):
        counter += 1
        insn = instructions[regs[ip]]
        parts = insn.split()
        ops = [None] + [int(x) for x in parts[1:]]
        old_ip = regs[ip] + 1

        if old_ip == 21 and regs[1] == 0:
            lhs = regs[2] // 256
            assert (lhs + 1) * 256 > regs[2]
            if lhs * 256 > regs[2]:
                lhs -= 1
            regs[1] = lhs
            regs[5] = (lhs + 1) * 256
            counter += lhs * 7

        before = regs.copy()
        regs[ops[3]] = opcodes[parts[0]](regs, ops)
        regs[ip] += 1
        if verbose:
            print(f'counter={counter} ip(#{ip})={old_ip}', before, '->', regs, '//', insn)
        if counter > limit:
            return None
        # assert old_ip != 29
    print('exit after', counter)
    return counter

best = 11840402
best_steps = 10e15

for i in range(best, -1, -1):
    assert i >= 0
    r = run(i, best_steps, False)
    print(i)
    if r:
        best = i
        best_steps = r
        print(best, best_steps)
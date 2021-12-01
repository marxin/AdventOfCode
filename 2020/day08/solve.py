#!/usr/bin/env python3

import copy

def run(lines):
    acc = 0
    i = 0
    seen = set()

    while True:
        if i == len(lines):
            print(acc)
            return True
        if i in seen:
            # part 1
            # print(acc)
            return False
        seen.add(i)
        insn = lines[i]
        parts = insn.split(' ')
        opcode = parts[0]
        if opcode == 'nop':
            i += 1
        elif opcode == 'acc':
            acc += int(parts[1])
            i += 1
        elif opcode == 'jmp':
            i += int(parts[1])
        else:
            assert False

lines = open('input.txt').read().splitlines()

for i in range(len(lines)):
    if lines[i].startswith('nop'):
        lines2 = copy.copy(lines)
        lines2[i] = lines2[i].replace('nop', 'jmp')
        run(lines2)
    if lines[i].startswith('jmp'):
        lines2 = copy.copy(lines)
        lines2[i] = lines2[i].replace('jmp', 'nop')
        run(lines2)

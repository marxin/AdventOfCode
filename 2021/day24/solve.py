#!/usr/bin/env python3

import os
import sys
import random

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

def get_val(registers, value):
    return int(value) if value not in registers else registers[value]

def run_program(lines, input):
    read = []
    registers = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    for line in lines:
        parts = line.split()
        opcode = parts[0]
        if opcode == 'inp':
            reg = parts[1]
            registers[reg] = input[0]
            read.append(input[0])
            input = input[1:]
        else:
            a, b = parts[1:]
            assert a != 'w'
            if opcode == 'add':
                registers[a] += get_val(registers, b)
            elif opcode == 'mul':
                registers[a] *= get_val(registers, b)
            elif opcode == 'div':
                registers[a] //= get_val(registers, b)
            elif opcode == 'mod':
                registers[a] %= get_val(registers, b)
            elif opcode == 'eql':
                registers[a] = 1 if get_val(registers, b) == registers[a] else 0
            else:
                assert False

    assert not input
    return registers['z']

L = 14

while True:
    digits = [random.randrange(1, 10) for _ in range(L)]
    print(digits)
    assert len(digits) == L
    r = run_program(lines, digits)
    if r == 0:
        print('DONE:', start)
        break
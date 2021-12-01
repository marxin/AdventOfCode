#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()
pc = 0
regs = {'a': 1, 'b': 0}

while pc < len(lines):
    print(pc)
    line = lines[pc]
    parts = line.split(' ')
    i = parts[0]
    if i == 'hlf':
        regs[parts[1]] = int(regs[parts[1]] / 2)
    elif i == 'tpl':
        regs[parts[1]] *= 3
    elif i == 'inc':
        regs[parts[1]] += 1
    elif i == 'jmp':
        pc += int(parts[1])
        continue
    elif i == 'jie':
        if regs[parts[1].strip(',')] % 2 == 0:
            pc += int(parts[2])
            continue
    elif i == 'jio':
        if regs[parts[1].strip(',')] == 1:
            pc += int(parts[2])
            continue
    else:
        assert False
    pc += 1

print(regs)

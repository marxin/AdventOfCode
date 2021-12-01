#!/usr/bin/env python3

from termcolor import colored

init_data = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0]

data = None
pc = None

def init():
    global data
    global pc
    data = init_data.copy()
    pc = 0

def print_data(index):
    s = ','.join([str(v) if i != index else colored(v, 'red') for i, v in enumerate(data)])
    print(s)

def process():
    global pc
    opcode = data[pc]
    if opcode == 1:
        # addition
        op0 = data[pc + 1]
        op1 = data[pc + 2]
        dest = data[pc + 3]
        data[dest] = data[op0] + data[op1]
        pc += 4
        return dest
    elif opcode == 2:
        # multiplication
        op0 = data[pc + 1]
        op1 = data[pc + 2]
        dest = data[pc + 3]
        data[dest] = data[op0] * data[op1]
        pc += 4
        return dest
    elif opcode == 99:
        return -1
    else:
        print('Unknown opcode: %d' % opcode)
        assert False

def run(noun, verb):
    init()
    data[1] = noun
    data[2] = verb

    step = 0
    # print('Initial data')
    # print_data(-1)
    while True:
        step += 1
        # print('Step %d:' % step)
        r = process()
        if r == -1:
            exit = data[0] 
            return exit
        else:
            # print_data(r)
            pass

print(run(12, 2))
needle = 19690720

for i in range(100):
    for j in range(100):
        e = run(i, j)
        if e == needle:
            print('%d:%d gives needle %d' % (i, j, needle))
            exit(0)

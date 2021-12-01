#!/usr/bin/env python3

import time
from termcolor import colored
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--verbose', '-v', help="Verbose mode", action = 'store_true')
args = parser.parse_args()

init_data = [109,424,203,1,21102,11,1,0,1105,1,282,21102,1,18,0,1106,0,259,2101,0,1,221,203,1,21102,1,31,0,1106,0,282,21102,38,1,0,1105,1,259,20101,0,23,2,22101,0,1,3,21101,1,0,1,21101,57,0,0,1105,1,303,2101,0,1,222,21001,221,0,3,21002,221,1,2,21101,0,259,1,21102,80,1,0,1106,0,225,21102,89,1,2,21102,91,1,0,1105,1,303,2101,0,1,223,20101,0,222,4,21101,0,259,3,21102,1,225,2,21102,225,1,1,21102,118,1,0,1106,0,225,20101,0,222,3,21101,136,0,2,21101,133,0,0,1106,0,303,21202,1,-1,1,22001,223,1,1,21101,148,0,0,1105,1,259,1202,1,1,223,20102,1,221,4,21001,222,0,3,21102,18,1,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,195,1,0,106,0,108,20207,1,223,2,20102,1,23,1,21101,-1,0,3,21101,214,0,0,1105,1,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1202,-4,1,249,21201,-3,0,1,22102,1,-2,2,21202,-1,1,3,21102,1,250,0,1105,1,225,21201,1,0,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2105,1,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,22102,1,-2,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,21201,-2,0,3,21102,343,1,0,1106,0,303,1105,1,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,21202,-4,1,1,21102,384,1,0,1105,1,303,1106,0,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,21202,1,1,-4,109,-5,2106,0,0]

moves = [(0, 1), (0, -1), (-1, 0), (1, 0)]
inverse_moves = {1: 2, 2: 1, 3: 4, 4:3}

opcode_names = {
        1: 'ADD',
        2: 'MULT',
        3: 'READ',
        4: 'WRITE',
        5: 'JMPEQ',
        6: 'JMPNEQ',
        7: 'LT',
        8: 'EQ',
        9: 'SET BASE',
        99: 'EXIT'
}

class Instruction:
    def __init__(self, program, insn):
        self.program = program 
        self.raw = insn
        self.modes = [0, 0, 0, 0]
        if insn >= 100:
            self.opcode = insn % 100
            modes = int(insn / 100)
            i = 0
            while modes > 0:
                self.modes[i] = modes % 10
                modes = int(modes / 10)
                i += 1
        else:
            self.opcode = insn

    def extend(self, index):
        while len(self.program.data) <= index:
            self.program.data.append(0)

    def get_arg(self, arg, mode_index):
        mode = self.modes[mode_index]
        index = None
        if mode == 0:
            index = self.program.data[arg]
        elif mode == 1:
            index = arg
        elif mode == 2:
            index = self.program.data[arg] + self.program.base
        else:
            assert False
        self.extend(index)
        if args.verbose:
            print('  getting at data[%d]=%d' % (index, self.program.data[index]))
        return self.program.data[index]

    def store(self, arg, mode_index, value):
        mode = self.modes[mode_index]
        index = None
        index = None
        if mode == 0:
            index = self.program.data[arg]
        elif mode == 1:
            assert False
        elif mode == 2:
            index = self.program.data[arg] + self.program.base
        else:
            assert False

        self.extend(index)
        self.program.data[index] = value
        if args.verbose:
            print('  storing at data[%d]=%d' % (index, value))
        return index

    def print(self):
        print('pc: %d, base: %d, raw: %d, opcode: %d (%s), modes: %s' % (self.program.pc, self.program.base, self.raw, self.opcode, opcode_names[self.opcode], str(self.modes)))

class Program:
    def __init__(self, data, input_values):
        self.data = data.copy()
        self.pc = 0
        self.base = 0
        self.last_modified = -1
        self.exit = False
        self.input_values = input_values
        self.output = None

    def print_data(self):
        for i, v in enumerate(self.data):
            if i == self.pc:
                print(colored('PC:', 'blue'), end = '')
            if i == self.last_modified:
                print(colored(v, 'red'), end = '')
            else:
                print(v, end = '')
            print(',', end = '')
        print()

    def process_one_instruction(self):
        self.last_modified = -1
        insn = Instruction(self, self.data[self.pc])
        if args.verbose:
            insn.print()
        if insn.opcode == 1:
            # ADD 
            r = insn.store(self.pc + 3, 2, insn.get_arg(self.pc + 1, 0) + insn.get_arg(self.pc + 2, 1))
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 2:
            # MULT
            r = insn.store(self.pc + 3, 2, insn.get_arg(self.pc + 1, 0) * insn.get_arg(self.pc + 2, 1))
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 3:
            # READ
            # print('Setting input to: %d' % self.last_input)
            input = self.input_values[0]
            self.input_values = self.input_values[1:]
            r = insn.store(self.pc + 1, 0, input)
            self.pc += 2
            self.last_modified = r
        elif insn.opcode == 4:
            # WRITE
            output = insn.get_arg(self.pc + 1, 0)
            if args.verbose:
                print(colored('Output: %d' % output, 'red'))
            self.output = output
            self.pc += 2
        elif insn.opcode == 5 or insn.opcode == 6:
            # EQ/NEQ
            cmp = insn.get_arg(self.pc + 1, 0)
            if insn.opcode == 6:
                cmp = not cmp
            if cmp:
                self.pc = insn.get_arg(self.pc + 2, 1)
            else:
                self.pc += 3
        elif insn.opcode == 7:
            # LT
            r = int(insn.get_arg(self.pc + 1, 0) < insn.get_arg(self.pc + 2, 1))
            r = insn.store(self.pc + 3, 2, r)
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 8:
            # EQ
            r = int(insn.get_arg(self.pc + 1, 0) == insn.get_arg(self.pc + 2, 1))
            r = insn.store(self.pc + 3, 2, r)
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 9:
            # SET BASE
            self.base += insn.get_arg(self.pc + 1, 0)
            self.pc += 2
        elif insn.opcode == 99:
            # print('Exit')
            self.exit = True
        else:
            print('Unknown opcode: %d' % insn.opcode)
            assert False

    def run(self):
        step = 0
        #self.print_data()
        self.last_output = None
        counter = 1
        while True:
            step += 1
            if args.verbose:
                print('Step %d:' % step)
            r = self.process_one_instruction()
            if args.verbose:
                self.print_data()
            if self.exit:
                return

def get_value(x, y):
    program = Program(init_data, [x, y])
    program.run()
    return program.output

width = 180
y = round(130.0 / 18 * width)
s = round(10.0 / 13 * y)
N = 100

print(y)
print(s)

for i in range(y, y + 100):
    line = ''
    for j in range(s, s + 170):
        v = get_value(i, j)
        line += '#' if v else '.'
    ri = s + line.rindex('#')
    print('%6s/%6s: %s' % (str(i), str(ri), line))
    t = (i + N - 1, ri - N + 1) 
    if get_value(t[0], t[1]):
        print('DONE')
        x = t[0] - N + 1
        y = t[1]
        print(x)
        print(y)
        print(10000 * x + y)
        print(10000 * y + x)
        exit(1)

#!/usr/bin/env python3

from termcolor import colored
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--verbose', '-v', help="Verbose mode", action = 'store_true')
args = parser.parse_args()

init_data = [3,8,1001,8,10,8,105,1,0,0,21,38,63,72,85,110,191,272,353,434,99999,3,9,102,4,9,9,101,2,9,9,102,3,9,9,4,9,99,3,9,1001,9,4,9,102,2,9,9,1001,9,5,9,1002,9,5,9,101,3,9,9,4,9,99,3,9,1001,9,2,9,4,9,99,3,9,1001,9,3,9,102,2,9,9,4,9,99,3,9,101,2,9,9,102,2,9,9,1001,9,2,9,1002,9,4,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99]

data = None
pc = None

def permutate(start, input, output):
    if len(input):
        for i in range(len(input)):
            start.append(input[i])
            permutate(start, input[:i] + input[i + 1:], output)
            start.pop()
    else:
        output.append(start.copy())

class Instruction:
    def __init__(self, data, insn):
        self.data = data
        self.raw = insn
        self.immediates = [False, False, False]
        if insn >= 100:
            self.opcode = insn % 100
            immediates = int(insn / 100)
            i = 0
            while immediates > 0:
                self.immediates[i] = bool(immediates % 10)
                immediates = int(immediates / 10)
                i += 1
        else:
            self.opcode = insn

    def get_arg(self, arg, index):
        if self.immediates[index]:
            return self.data[arg]
        else:
            return self.data[self.data[arg]]

    def store(self, index, value):
        self.data[index] = value
        return index

    def print(self):
        print('pc: %d, raw: %d, opcode: %d, immediates: %s' % (pc, self.raw, self.opcode, str(self.immediates)))

def init():
    global data
    global pc
    data = init_data.copy()
    pc = 0

last_output = None
input_values = None


class Program:
    def __init__(self, data, index):
        self.data = data.copy()
        self.pc = 0
        self.input_values = []
        self.last_output = None
        self.last_modified = -1
        self.index = index
        self.exit = False

    def print_data(self):
        s = ','.join([str(v) if i != self.last_modified else colored(v, 'red') for i, v in enumerate(self.data)])
        print(s)

    def process_one_instruction(self):
        self.last_modified = -1
        insn = Instruction(self.data, self.data[self.pc])
        if args.verbose:
            insn.print()
        if insn.opcode == 1:
            # addition
            r = insn.store(self.data[self.pc + 3], insn.get_arg(self.pc + 1, 0) + insn.get_arg(self.pc + 2, 1))
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 2:
            # multiplication
            r = insn.store(self.data[self.pc + 3], insn.get_arg(self.pc + 1, 0) * insn.get_arg(self.pc + 2, 1))
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 3:
            insn.immediates[0] = True
            input_value = self.input_values.pop()
            print('Setting input to: %d' % input_value)
            r = insn.store(insn.get_arg(self.pc + 1, 0), input_value)
            self.pc += 2
            self.last_modified = r
        elif insn.opcode == 5 or insn.opcode == 6:
            cmp = insn.get_arg(self.pc + 1, 0)
            if insn.opcode == 6:
                cmp = not cmp
            if cmp:
                self.pc = insn.get_arg(self.pc + 2, 1)
            else:
                self.pc += 3
        elif insn.opcode == 7:
            r = int(insn.get_arg(self.pc + 1, 0) < insn.get_arg(self.pc + 2, 1))
            r = insn.store(self.data[self.pc + 3], r)
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 8:
            r = int(insn.get_arg(self.pc + 1, 0) == insn.get_arg(self.pc + 2, 1))
            r = insn.store(self.data[self.pc + 3], r)
            self.pc += 4
            self.last_modified = r
        elif insn.opcode == 4:
            output = insn.get_arg(self.pc + 1, 0)
            print(colored('Output: %d' % output, 'red'))
            self.last_output = output
            self.pc += 2
        elif insn.opcode == 99:
            print('Exit')
            self.exit = True
        else:
            print('Unknown opcode: %d' % insn.opcode)
            assert False

    def run(self):
        step = 0
        self.print_data()
        self.last_output = None
        while True:
            step += 1
            if args.verbose:
                print('Step %d:' % step)
            r = self.process_one_instruction()
            if args.verbose:
                print_data()
            if self.exit:
                return self.last_output
            if self.last_output != None:
                print('Last output: %d and returning' % self.last_output)
                return self.last_output

def one_run(permutation):
    runindex = 0
    amplifiers = [Program(init_data.copy(), v) for v in permutation]
    last_output = 0
    loop_done = False

    while True:
        if loop_done:
            amplifiers[runindex].input_values = [last_output]
        else:
            amplifiers[runindex].input_values = [last_output, amplifiers[runindex].index]
        print('Starting A#%d' % runindex)
        last_output = amplifiers[runindex].run()
        if amplifiers[runindex].exit:
            return amplifiers[-1].last_output
        runindex = (runindex + 1) % 5
        if runindex == 0:
            loop_done = True

permutations = []
permutate([], list(range(5, 10)), permutations)

max = 0
max_permutation = None
for p in permutations:
    r = one_run(p)
    if r > max:
        max = r
        max_permutation = p

print(max)

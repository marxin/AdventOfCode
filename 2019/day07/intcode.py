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
    def __init__(self, insn):
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
            return data[arg]
        else:
            return data[data[arg]]

    def store(self, index, value):
        data[index] = value
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

def print_data(index):
    s = ','.join([str(v) if i != index else colored(v, 'red') for i, v in enumerate(data)])
    print(s)

def process():
    global pc
    global last_output
    global input_values
    insn = Instruction(data[pc])
    if args.verbose:
        insn.print()
    if insn.opcode == 1:
        # addition
        r = insn.store(data[pc + 3], insn.get_arg(pc + 1, 0) + insn.get_arg(pc + 2, 1))
        pc += 4
        return r
    elif insn.opcode == 2:
        # multiplication
        r = insn.store(data[pc + 3], insn.get_arg(pc + 1, 0) * insn.get_arg(pc + 2, 1))
        pc += 4
        return r
    elif insn.opcode == 3:
        insn.immediates[0] = True
        input_value = input_values.pop()
        print('Setting input to: %d' % input_value)
        r = insn.store(insn.get_arg(pc + 1, 0), input_value)
        pc += 2
        return r
    elif insn.opcode == 5 or insn.opcode == 6:
        cmp = insn.get_arg(pc + 1, 0)
        if insn.opcode == 6:
            cmp = not cmp
        if cmp:
            pc = insn.get_arg(pc + 2, 1)
        else:
            pc += 3
        return -1
    elif insn.opcode == 7:
        r = int(insn.get_arg(pc + 1, 0) < insn.get_arg(pc + 2, 1))
        r = insn.store(data[pc + 3], r)
        pc += 4
        return r
    elif insn.opcode == 8:
        r = int(insn.get_arg(pc + 1, 0) == insn.get_arg(pc + 2, 1))
        r = insn.store(data[pc + 3], r)
        pc += 4
        return r
    elif insn.opcode == 4:
        output = insn.get_arg(pc + 1, 0)
        print(colored('Output: %d' % output, 'red'))
        last_output = output
        pc += 2
        return -1
    elif insn.opcode == 99:
        print('Exit')
        return -99
    else:
        print('Unknown opcode: %d' % insn.opcode)
        assert False

def run(permutation):
    global last_output
    global input_values
    last_output = 0

    while len(permutation):
        init()
        step = 0
        print_data(-1)
        input_values = [last_output, permutation[0]]
        permutation = permutation[1:]
        while True:
            step += 1
            if args.verbose:
                print('Step %d:' % step)
            r = process()
            if args.verbose:
                print_data(r)
            if r == -99:
                break

    print('Last output: %d' % last_output)
    return last_output

permutations = []
permutate([], list(range(5)), permutations)

max = 0
max_permutation = None
for p in permutations:
    r = run(p)
    if r > max:
        max = r
        max_permutation = p

print(max)
print(max_permutation)

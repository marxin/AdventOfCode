#!/usr/bin/env python3

from termcolor import colored
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--verbose', '-v', help="Verbose mode", action = 'store_true')
args = parser.parse_args()

init_data = [3,225,1,225,6,6,1100,1,238,225,104,0,1101,90,60,224,1001,224,-150,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1,57,83,224,1001,224,-99,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1102,92,88,225,101,41,187,224,1001,224,-82,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1101,7,20,225,1101,82,64,225,1002,183,42,224,101,-1554,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1102,70,30,224,101,-2100,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,2,87,214,224,1001,224,-2460,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,102,36,180,224,1001,224,-1368,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1102,50,38,225,1102,37,14,225,1101,41,20,225,1001,217,7,224,101,-25,224,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,1101,7,30,225,1102,18,16,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,226,226,224,102,2,223,223,1006,224,329,101,1,223,223,1107,677,226,224,102,2,223,223,1006,224,344,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,359,101,1,223,223,107,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,108,677,226,224,1002,223,2,223,1005,224,404,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,8,226,677,224,102,2,223,223,1006,224,434,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,449,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,464,101,1,223,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,494,1001,223,1,223,8,677,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1108,677,677,224,102,2,223,223,1005,224,524,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,539,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,554,1001,223,1,223,1007,226,226,224,102,2,223,223,1005,224,569,1001,223,1,223,1008,226,226,224,102,2,223,223,1005,224,584,101,1,223,223,1007,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,108,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1007,226,677,224,1002,223,2,223,1006,224,629,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,644,101,1,223,223,1107,226,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226]

data = None
pc = None

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

def print_data(index):
    s = ','.join([str(v) if i != index else colored(v, 'red') for i, v in enumerate(data)])
    print(s)

def process():
    global pc
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
        input_value = 5
        print('Getting hardcoded input %d' % input_value)
        # input_value = int(input('Get input:'))
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
        print(colored('Output: %d' % insn.get_arg(pc + 1, 0), 'red'))
        pc += 2
    elif insn.opcode == 99:
        print('Exit')
        exit(0)
    else:
        print('Unknown opcode: %d' % insn.opcode)
        assert False

def run():
    init()
    step = 0
    print('Initial data')
    print_data(-1)
    while True:
        step += 1
        if args.verbose:
            print('Step %d:' % step)
        r = process()
        if args.verbose:
            print_data(r)

print(run())

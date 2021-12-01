#!/usr/bin/env python3

from termcolor import colored
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--verbose', '-v', help="Verbose mode", action = 'store_true')
args = parser.parse_args()

init_data = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
#init_data = [1102,34915192,34915192,7,4,7,99,0]
#init_data = [104,1125899906842624,99]
init_data = [1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,0,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,38,1019,1102,1,37,1008,1101,252,0,1023,1102,24,1,1004,1102,35,1,1017,1101,0,28,1011,1101,0,36,1003,1102,30,1,1013,1101,0,0,1020,1102,1,1,1021,1102,897,1,1028,1101,20,0,1000,1101,0,22,1005,1102,29,1,1007,1101,0,34,1009,1102,1,259,1022,1101,310,0,1025,1102,892,1,1029,1101,21,0,1014,1102,1,315,1024,1101,0,33,1002,1102,31,1,1015,1102,190,1,1027,1102,1,39,1001,1101,26,0,1010,1101,27,0,1016,1102,1,23,1018,1101,0,32,1012,1101,0,25,1006,1102,1,197,1026,109,34,2106,0,-7,1001,64,1,64,1106,0,199,4,187,1002,64,2,64,109,-22,2108,34,-3,63,1005,63,221,4,205,1001,64,1,64,1106,0,221,1002,64,2,64,109,-10,1208,-1,42,63,1005,63,237,1106,0,243,4,227,1001,64,1,64,1002,64,2,64,109,20,2105,1,1,1001,64,1,64,1105,1,261,4,249,1002,64,2,64,109,1,21108,40,40,-6,1005,1017,283,4,267,1001,64,1,64,1105,1,283,1002,64,2,64,109,7,1205,-9,301,4,289,1001,64,1,64,1105,1,301,1002,64,2,64,109,-1,2105,1,-5,4,307,1106,0,319,1001,64,1,64,1002,64,2,64,109,-8,1206,0,331,1105,1,337,4,325,1001,64,1,64,1002,64,2,64,109,-6,21108,41,38,0,1005,1015,353,1105,1,359,4,343,1001,64,1,64,1002,64,2,64,109,11,1206,-6,377,4,365,1001,64,1,64,1106,0,377,1002,64,2,64,109,1,21101,42,0,-8,1008,1019,42,63,1005,63,399,4,383,1105,1,403,1001,64,1,64,1002,64,2,64,109,-29,1202,6,1,63,1008,63,24,63,1005,63,425,4,409,1106,0,429,1001,64,1,64,1002,64,2,64,109,14,1201,-3,0,63,1008,63,34,63,1005,63,451,4,435,1105,1,455,1001,64,1,64,1002,64,2,64,109,10,21101,43,0,-9,1008,1013,41,63,1005,63,475,1106,0,481,4,461,1001,64,1,64,1002,64,2,64,109,-17,2101,0,0,63,1008,63,21,63,1005,63,501,1106,0,507,4,487,1001,64,1,64,1002,64,2,64,109,-5,2107,21,5,63,1005,63,525,4,513,1105,1,529,1001,64,1,64,1002,64,2,64,109,13,1202,-7,1,63,1008,63,26,63,1005,63,553,1001,64,1,64,1106,0,555,4,535,1002,64,2,64,109,5,21107,44,45,-8,1005,1010,573,4,561,1105,1,577,1001,64,1,64,1002,64,2,64,109,-6,21102,45,1,7,1008,1019,45,63,1005,63,603,4,583,1001,64,1,64,1105,1,603,1002,64,2,64,109,-15,1207,10,28,63,1005,63,623,1001,64,1,64,1106,0,625,4,609,1002,64,2,64,109,8,2108,37,-4,63,1005,63,645,1001,64,1,64,1105,1,647,4,631,1002,64,2,64,109,6,21102,46,1,1,1008,1012,44,63,1005,63,671,1001,64,1,64,1106,0,673,4,653,1002,64,2,64,109,4,1207,-6,35,63,1005,63,695,4,679,1001,64,1,64,1106,0,695,1002,64,2,64,109,1,2107,38,-8,63,1005,63,715,1001,64,1,64,1105,1,717,4,701,1002,64,2,64,109,-23,1208,10,36,63,1005,63,739,4,723,1001,64,1,64,1105,1,739,1002,64,2,64,109,4,2102,1,7,63,1008,63,24,63,1005,63,765,4,745,1001,64,1,64,1105,1,765,1002,64,2,64,109,13,2102,1,-4,63,1008,63,22,63,1005,63,789,1001,64,1,64,1105,1,791,4,771,1002,64,2,64,109,-8,1201,5,0,63,1008,63,32,63,1005,63,811,1106,0,817,4,797,1001,64,1,64,1002,64,2,64,109,11,1205,7,829,1105,1,835,4,823,1001,64,1,64,1002,64,2,64,109,-1,2101,0,-6,63,1008,63,25,63,1005,63,857,4,841,1106,0,861,1001,64,1,64,1002,64,2,64,109,8,21107,47,46,-9,1005,1011,877,1106,0,883,4,867,1001,64,1,64,1002,64,2,64,109,9,2106,0,-1,4,889,1106,0,901,1001,64,1,64,4,64,99,21101,0,27,1,21102,915,1,0,1105,1,922,21201,1,59500,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1105,1,922,21201,1,0,-1,21201,-2,-3,1,21101,0,957,0,1105,1,922,22201,1,-1,-2,1105,1,968,21201,-2,0,-2,109,-3,2105,1,0]

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
    def __init__(self, data):
        self.data = data.copy()
        self.pc = 0
        self.base = 0
        self.input_values = [2]
        self.last_output = None
        self.last_modified = -1
        self.exit = False

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
            input_value = self.input_values.pop()
            print('Setting input to: %d' % input_value)
            r = insn.store(self.pc + 1, 0, input_value)
            self.pc += 2
            self.last_modified = r
        elif insn.opcode == 4:
            # WRITE
            output = insn.get_arg(self.pc + 1, 0)
            print(colored('Output: %d' % output, 'red'))
            self.last_output = output
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
                self.print_data()
            if self.exit:
                return self.last_output

Program(init_data).run()

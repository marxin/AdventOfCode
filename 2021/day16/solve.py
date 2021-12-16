#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter
from math import prod

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

class Packet:
    def __init__(self, version, type, value=None):
        self.value = value
        self.version = version
        self.type = type
        self.children = []

    def get_version_sum(self):
        return self.version + sum(x.get_version_sum() for x in self.children)

    def get_sum(self):
        subvalues = [x.get_sum() for x in self.children]
        if self.type == 4:
            return self.value
        elif self.type == 0:
            return sum(subvalues)
        elif self.type == 1:
            return prod(subvalues)
        elif self.type == 2:
            return min(subvalues)
        elif self.type == 3:
            return max(subvalues)
        elif self.type == 5:
            return 1 if subvalues[0] > subvalues[1] else 0
        elif self.type == 6:
            return 1 if subvalues[0] < subvalues[1] else 0
        elif self.type == 7:
            return 1 if subvalues[0] == subvalues[1] else 0
        else:
            assert False

    def __repr__(self):
        return f'{self.version}/{self.value}: {self.children}'

def tohex(line):
    output = ''
    for c in line:
        v = int(c, 16)
        output += bin(v)[2:].zfill(4)
    return output

def parse_line(line, parsen):
    elements = []

    while True:
        if line == '0' * len(line):
            break
        version = int(line[:3], 2)
        type = int(line[3:6], 2)
        line = line[6:]

        if type == 4:
            literal = ''
            while True:
                stop = line[0] == '0'
                literal += line[1:5]
                line = line[5:]

                if stop:
                    break
            
            elements.append(Packet(version, type, int(literal, 2)))
            parsen -= 1
            if parsen == 0:
                break
        else:
            ltype = line[0]
            line = line[1:]
            p = Packet(version, type)
            if ltype == '0':
                length = int(line[:15], 2)
                line = line[15:]
                
                chunk = line[:length]
                p.children, _ = parse_line(chunk, 0)
                line = line[length:]
            else:
                subpackets = int(line[:11], 2)
                line = line[11:]
                p.children, line = parse_line(line, subpackets)

            elements.append(p)
            parsen -= 1
            if parsen == 0:
                break
    
    return (elements, line)


for line in lines:
    hex = tohex(line)
    elements, _ = parse_line(hex, 0)
    root = elements[0]
    print(root)
    print(root.get_version_sum(), root.get_sum())

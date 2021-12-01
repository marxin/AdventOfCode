#!/usr/bin/env python3

BASE = 16

data = open('input.txt').read()
lines = data.splitlines()

values = {}
wires = {}

def get_value(name):
    if name.isdigit():
        return int(name)
    else:
        return values[name]

def process(lhs, rhs):
    parts = lhs.split(' ')
    if len(parts) == 1:
        v1 = get_value(parts[0])
        if v1 == None:
            return False
        values[rhs] = v1
    elif len(parts) == 2:
        assert parts[0] == 'NOT'
        v1 = get_value(parts[1])
        if v1 == None:
            return False
        values[rhs] = 2**BASE + ~v1
    elif len(parts) == 3:
        v0 = get_value(parts[0])
        op1 = parts[1]
        v2 = get_value(parts[2])
        if v0 == None or v2 == None:
            return False
        if op1 == 'AND':
            values[rhs] = v0 & v2
        elif op1 == 'OR':
            values[rhs] = v0 | v2
        elif op1 == 'LSHIFT':
            values[rhs] = v0 << v2
        elif op1 == 'RSHIFT':
            values[rhs] = v0 >> v2
        else:
            assert False
    else:
        assert False
    return True

for line in lines:
    lhs, rhs = line.split(' -> ')
    values[rhs] = None
    wires[rhs] = lhs
    if lhs.isdigit():
        values[rhs] = int(lhs)

while wires:
    for k, v in list(wires.items()):
        if process(v, k):
            assert values[k] != None
            del wires[k]

print(values['a'])

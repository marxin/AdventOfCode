#!/usr/bin/env python3

BITS = 36

data = open('input.txt').read()
lines = data.splitlines()

mask_and = None
mask_or = None
mem = {}

def generate_addresses(indexstr, addresses):
    if 'X' in indexstr:
        i = indexstr.index('X')
        indexstr[i] = '0'
        generate_addresses(indexstr, addresses)
        indexstr[i] = '1'
        generate_addresses(indexstr, addresses)
        indexstr[i] = 'X'
    else:
        addresses.append(''.join(indexstr))

def combine(mask, index):
    index = bin(index)[2:].rjust(BITS, '0')
    assert len(index) == len(mask)
    index = list(index)
    for i, v in enumerate(mask):
        if v == '0':
            pass
        elif v == '1':
            index[i] = '1'
        elif v == 'X':
            index[i] = 'X'
        else:
            assert False
    addresses = []
    generate_addresses(index, addresses)
    return addresses

mask = None

for line in lines:
    lhs, rhs = line.split(' = ')
    if lhs == 'mask':
        assert len(rhs) == BITS
        mask = rhs
    else:
        assert lhs.startswith('mem')
        index = int(lhs.replace('[', ']').split(']')[1])
        value = int(rhs)
        for address in combine(mask, index):
            mem[int(address, 2)] = value

print(sum(mem.values()))

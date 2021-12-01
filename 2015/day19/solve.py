#!/usr/bin/env python3

import random
import sys

rulestxt = '''
Al => ThF
Al => ThRnFAr
B => BCa
B => TiB
B => TiRnFAr
Ca => CaCa
Ca => PB
Ca => PRnFAr
Ca => SiRnFYFAr
Ca => SiRnMgAr
Ca => SiTh
F => CaF
F => PMg
F => SiAl
H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
H => HCa
H => NRnFYFAr
H => NRnMgAr
H => NTh
H => OB
H => ORnFAr
Mg => BF
Mg => TiMg
N => CRnFAr
N => HSi
O => CRnFYFAr
O => CRnMgAr
O => HP
O => NRnFAr
O => OTi
P => CaP
P => PTi
P => SiRnFAr
Si => CaSi
Th => ThCa
Ti => BP
Ti => TiTi
e => HF
e => NAl
e => OMg
'''

source = 'CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr'

inlhs = set()
rules = []
generated = set()

for line in rulestxt.strip().splitlines():
    lhs, rhs = line.split(' => ')
    rules.append((lhs, rhs))
    inlhs.add(lhs)

print(rules)

def find_indices(string, needle):
    i = 0
    while needle in string:
        index = string.find(needle)
        offset = index+len(needle)
        string = string[offset:]
        yield index + i
        i += offset

def tokenize(s):
    while s:
        token = s[0]
        s = s[1:]
        if s and s[0].islower():
            token += s[0]
            s = s[1:]
        yield token

for x in find_indices(source, 'C'):
    assert source[x] == 'C'

for lhs, rhs in rules:
    for i in find_indices(source, lhs):
        generated.add(source[:i] + rhs + source[i + len(lhs):])

print(len(generated))

rules = sorted(rules, key=lambda x: len(x[1]), reverse=True)
print(rules)

seen = set()

minimal = 10000

called = 0
hits = 0

def shrinken(string, replacements):
    global minimal
    if string == 'e':
        print(replacements)
        sys.exit(0)
    if len(string) < minimal:
        minimal = len(string)
        print(minimal)
        print(string)
    random.shuffle(rules)
    for lhs, rhs in rules:
        indices = list(find_indices(string, rhs))
        random.shuffle(indices)
        for i in indices:
            shrinken(string[:i] + lhs + string[i+len(rhs):], replacements + 1)

shrinken(source, 0)

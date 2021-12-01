#!/usr/bin/env python3

data = open('input.txt').read()
lines = data.splitlines()

def findstart(i):
    v = tokens[i]
    pare = 1 if v == ')' else 0
    while pare != 0:
        i -= 1
        v = tokens[i]
        if v == ')':
            pare += 1
        if v == '(':
            pare -= 1
    return i

def findend(i):
    v = tokens[i]
    pare = 1 if v == '(' else 0
    while pare != 0:
        i += 1
        v = tokens[i]
        if v == '(':
            pare += 1
        if v == ')':
            pare -= 1
    return i


def encapsulate():
    i = 0
    while i < len(tokens):
        v = tokens[i]
        if v == '+' and (tokens[i - 1] != '(' or tokens[i + 1] != ')'):
            tokens.insert(findend(i + 1) + 1, ')')
            tokens.insert(findstart(i - 1), '(')
            i += 1
        i += 1

suma = 0

for line in lines:
    tokens = [t for t in line.replace('(', ' ( ').replace(')', ' ) ').split(' ') if t]
    encapsulate()
    string = ''.join(tokens)
    # print(string)
    result = eval(string)
    suma += result
    # print(result)

print(suma)

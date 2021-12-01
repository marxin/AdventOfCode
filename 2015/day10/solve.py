#!/usr/bin/env python3

data = open('input.txt').read().rstrip()

def leading(string, start):
    c = string[start]
    for i in range(start, len(string)):
        if c != string[i]:
            return i - start
    return len(string) - start

def readstr(string):
    for i in range(50):
        print(i, len(string))
        output = []
        j = 0
        while j < len(string):
            start = string[j]
            length = leading(string, j)
            output.append(str(length) + start)
            j += length
        string = ''.join(output)
    return string

output = readstr(data)
print(len(output))

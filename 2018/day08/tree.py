#!/usr/bin/env python3

numbers = list(reversed([int(x) for x in open('input.txt').read().split(' ')]))
metadata = []

class Node:
    def __init__(self):
        self.children = []
        self.metadata = []
        self.value = None

    def calculate(self):
        if self.children:
            self.value = 0
            for c in self.children:
                c.calculate()
            for m in self.metadata:
                if m >= 1 and m <= len(self.children):
                    self.value += self.children[m - 1].value
        else:
            self.value = sum(self.metadata)

def read_node(numbers):
    children = numbers.pop()
    n = Node()
    meta = numbers.pop()
    for i in range(children):
        n.children.append(read_node(numbers))
    for i in range(meta):        
        n.metadata.append(numbers.pop())
    return n

root = read_node(numbers)
print(sum(metadata))
print(root)
root.calculate()
print(root.value)

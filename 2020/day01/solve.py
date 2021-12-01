#!/usr/bin/env python3

numbers = [int(x) for x in open('input.txt').readlines()]

def part1():
    for i in range(len(numbers)):
        for j in range(len(numbers)):
            if i != j and numbers[i] + numbers[j] == 2020:
                print(numbers[i] * numbers[j])
                return

def part2():
    for i in range(len(numbers)):
        for j in range(len(numbers)):
            for k in range(len(numbers)):
                if i != j and j != k and numbers[i] + numbers[j] + numbers[k] == 2020:
                    print(numbers[i] * numbers[j] * numbers[k])
                    return

part1()
part2()

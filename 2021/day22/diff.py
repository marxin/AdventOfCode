#!/usr/bin/env python3

import subprocess
import sys

a = subprocess.check_output('/home/marxin/Programming/AdventOfCode/2021/day22/solve.py', shell=True, encoding='utf8').splitlines()[-1]
b = subprocess.check_output('/home/marxin/Programming/AdventOfCode/2021/day22/solve3.py', shell=True, encoding='utf8').splitlines()[-1]

print(a, b, a == b)
sys.exit(0 if a != b else 1)

#!/usr/bin/env python3

import os
import sys

from collections import defaultdict, Counter

folder = os.path.dirname(os.path.abspath(__file__))
data = open(os.path.join(folder, 'input.txt')).read()
lines = data.splitlines()

print(lines)

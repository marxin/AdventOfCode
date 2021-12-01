#!/usr/bin/env python3

import json

data = json.load(open('input.txt'))

total = 0

def countit(data):
    global total
    if isinstance(data, dict):
        if 'red' in data.values():
            return
        for k, v in data.items():
            assert not k.isdigit()
            countit(v)
    elif isinstance(data, list):
        for x in data:
            countit(x)
    elif isinstance(data, int):
        total += data

countit(data)
print(total)

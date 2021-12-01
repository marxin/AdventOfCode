#!/usr/bin/env python3

from math import ceil

data='''
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
'''

data2 = '''
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
'''

data3 = '''
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
'''

data4 = '''
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
'''

mydata = '''
5 HLJD, 1 QHSZD, 13 SKZX => 8 MQPH
10 LSLV => 4 JNJHW
1 MQGF, 4 ZWXDQ, 1 GNSZ => 9 DGDH
1 SKZX, 3 DJSP => 1 MCHV
6 TWSR, 10 ZHDFS, 10 LQZXQ => 9 LXQNX
1 FRVW, 1 CJTW => 9 BRCB
20 ZHVNP => 8 XMXL
7 JQJXP => 1 ZGZDW
13 KRCM => 6 KXPQ
4 ZWXDQ, 4 KFKQF, 1 DZDX => 2 MQGF
8 DZDX, 2 ZKGM => 3 KFKQF
3 FXFTB => 8 KVDGP
10 MVGLF, 3 MWFBW, 13 XMXL, 1 CJTW, 2 ZSXJZ, 2 TNCZH, 3 MPFKN, 6 LXQNX => 2 MZMZQ
5 FRVW => 3 NWBTP
1 MVGLF, 2 NLXD, 6 KVDGP, 2 MQPH, 4 FXTJ, 10 TKXKF, 2 FRWV => 2 CSNS
13 TWSR => 9 BNWT
2 KRCM => 7 LSLV
1 ZHDFS, 11 NTVZD, 1 JQJXP => 6 ZHVNP
2 MCHV, 1 JNJHW => 6 NDQNH
32 SMHJH, 6 KXPQ => 1 CJTW
15 FXFTB, 1 MVGLF => 9 MPFKN
119 ORE => 9 KRCM
3 TNCZH => 9 BFQLT
5 MPFKN, 7 TKXKF, 6 JQJXP, 2 DZDX, 16 LCQJ, 4 DGDH, 4 ZGZDW => 7 WVXW
1 ZHDFS, 1 LXQNX => 3 TNCZH
4 ZMVKM, 1 BRQT => 3 QHSZD
24 FRVW, 1 KVDGP, 2 ZLNM => 3 FGLNK
2 KXPQ, 1 LSLV, 22 HNRQ => 5 ZWXDQ
6 ZWXDQ => 1 FRVW
1 FXFTB, 2 MWFBW => 6 ZHDFS
32 FRVW => 5 FRWV
6 FXFTB, 6 NDQNH, 2 MWFBW => 1 JQJXP
9 ZMVKM, 6 QHSZD, 5 LSLV => 4 SMHJH
3 CHKZ => 6 HLJD
21 BFQLT => 6 FXTJ
1 SMHJH, 4 FXFTB => 6 CHKZ
13 FRVW, 13 JQJXP, 1 GNSZ => 8 ZSXJZ
2 NDQNH => 8 NTVZD
3 KRCM => 2 ZKGM
13 ZHDFS, 14 ZWXDQ, 1 CHKZ => 7 LQZXQ
2 BNWT, 3 CHKZ => 7 ZLNM
167 ORE => 1 BRQT
1 LSLV => 3 DZDX
8 MZMZQ, 7 NWBTP, 3 WVXW, 44 MQPH, 3 DJSP, 1 CSNS, 3 BRCB, 32 LQZXQ => 1 FUEL
8 ZLNM => 2 NLXD
30 JQJXP, 9 FGLNK => 7 LCQJ
1 ZKGM, 19 KXPQ => 8 DJSP
4 DJSP => 6 FXFTB
25 NFTPZ => 6 ZMVKM
14 ZHVNP, 1 MVGLF => 9 TKXKF
1 BRQT => 2 SKZX
6 ZKGM => 7 HNRQ
3 DZDX => 5 TWSR
1 SMHJH => 7 MVGLF
3 NDQNH => 1 GNSZ
153 ORE => 9 NFTPZ
14 MCHV, 4 JNJHW, 2 DJSP => 4 MWFBW
'''

test = '''
4 ORE => 4 B
3 ORE => 1 B
5 B => 1 FUEL
'''

class Formula:
    def __init__(self, formula):
        self.formula = formula
        parts = formula.split('=>')
        self.lhs = Formula.parse(parts[0])
        rhs = list(Formula.parse(parts[1]).items())
        assert len(rhs) == 1
        self.rhs = rhs[0]

    @staticmethod
    def parse(part):
        components = {}
        subparts = [p for p in part.split(',')]
        for s in subparts:
            parts = s.strip().split(' ')
            components[parts[1]] = int(parts[0])
        return components

maximum = 2**32

def sum(d1, d2):
    for k, v in d2.items():
        if k in d1:
            d1[k] += v
        else:
            d1[k] = v

def del_empty(d):
    for k, v in list(d.items()):
        if v == 0:
            del d[k]


def subtract_has(todo, has):
    for k, v in has.items():
        if k in todo:
            s = min(v, todo[k])
            todo[k] -= s
            has[k] -= s
    del_empty(todo)
    del_empty(has)

seen = {}

count = 0
seen_hit = 0

seen = {}

# d1 >= d2
def great_or_equal(d1, d2):
    for k, v in d1.items():
        if d2[k] < v:
            return False
    return True

def is_cached_state(todo, has, ores):
    global seen
    global seen_hit
    todo_keys = list(sorted(todo.keys()))
    has_keys = list(sorted(has.keys()))
    t = (frozenset(todo_keys), frozenset(has_keys))
    if t in seen:
        cache = seen[t]
        if cache[2] <= ores:
            if todo == cache[0] and has == cache[1]:
                seen_hit += 1
                return True
            # print('TODO: %s, cache[0]: %s, ores: %d/%d' % (str(todo), str(cache[0]), ores, cache[2]))
            if great_or_equal(todo, cache[0]) and great_or_equal(has, cache[1]):
                seen_hit += 1
                return True
        if ores < cache[2]:
            if great_or_equal(cache[0], todo) and great_or_equal(cache[1], has):
                seen[t] = (todo.copy(), has.copy(), ores)
                #print('can improve0: %d from %d' % (ores, cache[2]))
                #print('can improve: %s to %s' % (todo, cache[0]))
                #print('can improve2: %s to %s' % (has, cache[1]))
    else:
        seen[t] = (todo.copy(), has.copy(), ores)
    return False

seen2 = {}
seen2_hit = 0
def is_cached_state2(todo, has, ores):
    global seen2
    global seen2_hit

    t = (frozenset(todo), frozenset(has))
    if t in seen2:
        if ores >= seen2[t]:
            seen2_hit += 1
            return True
        else:
            seen2[t] = ores
    else:
        seen2[t] = ores
    return False

def resolve(todo, has, formulas, ores):
    global count
    global maximum
    global seen
    count += 1

    if count % 10000 == 0:
        print(count)
    if is_cached_state2(todo, has, ores):
        return
    if is_cached_state(todo, has, ores):
        return
    if ores > maximum:
        # print('bail out with: %d' % ores)
        return
    if not len(list(todo.items())):
        if ores < maximum:
            print('done with %d' % ores)
            maximum = ores
        return
    for todo_name, todo_value in list(todo.items()):
        for formula in formulas:
            if formula.rhs[0] == todo_name:
                #print('Before: %s, %s, ores: %d' % (str(todo), str(has), ores))
                saved_todo = todo.copy()
                saved_has = has.copy()
                saved_ores = ores
                times = max(1, ceil(todo_value / formula.rhs[1]))
                diff = times * formula.rhs[1] - todo_value
                if diff > 0:
                    has[formula.rhs[0]] = diff
                del todo[todo_name]
                newtodo = dict([(k, times * v) for k, v in formula.lhs.items()])
                sum(todo, newtodo)
                subtract_has(todo, has)
                if 'ORE' in todo:
                    ores += todo['ORE']
                    del todo['ORE']
                #print('Using: %s with k=%d (ores: %d)' % (formula.formula, times, ores))
                # print('After: %s, %s, ores: %d' % (str(todo), str(has), ores))
                resolve(todo, has, formulas, ores)
                todo = saved_todo
                has = saved_has
                ores = saved_ores

formulas = []
for line in mydata.strip().split('\n'):
    formulas.append(Formula(line))

resolve({'FUEL': 1}, {}, formulas, 0)

print('Iterations: %d, cached: %d' % (count, seen2_hit))
print('Minimum: %d' % maximum)

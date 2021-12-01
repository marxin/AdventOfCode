#!/usr/bin/env python3

me2 = [10, 0, 0, 250]
boss2 = [14, 8, 0]

me2 = [50, 0, 0, 500]
boss2 = [58, 9, 0]

spells = ['missile', 'drain', 'shield', 'poison', 'recharge']
spell_costs = [53, 73, 113, 173, 229]
spell_turns = [1, 1, 6, 6, 5]

def damage(damage, defender):
    d = max(1, damage - defender[2])
    defender[0] -= d

def play_spell(me, boss, spell):
    if spell == 'missile':
        damage(4, boss)
    elif spell == 'drain':
        damage(2, boss)
        me[0] += 2
    elif spell == 'shield':
        if me[2] == 0:
            me[2] = 7
    elif spell == 'poison':
        damage(3, boss)
    elif spell == 'recharge':
        me[3] += 101
    else:
        assert False

def decrement(active_spells, me):
    for k in list(active_spells):
        active_spells[k] -= 1
        if active_spells[k] == 0:
            del active_spells[k]
            if k == 'shield':
                me[2] = 0

def play_spells(me, boss, active_spells):
    for k in active_spells:
        play_spell(me, boss, k)

minimum = 10**10

def play(me, boss, active_spells, spent, played_spells):
    # print(played_spells, me, boss)
    global minimum

    if spent >= minimum:
        return False
    # ME
    me[0] -= 1
    if me[0] <= 0:
        return False
    play_spells(me, boss, active_spells)
    decrement(active_spells, me)
    if boss[0] <= 0:
        if spent < minimum:
            minimum = spent
            print(minimum, played_spells)
        return True

    for i in range(len(spells)):
        if spells[i] not in active_spells and spell_costs[i] <= me[3]:
            nextme = me.copy()
            nextboss = boss.copy()
            nextme[3] -= spell_costs[i]
            nextspent = spent + spell_costs[i]
            next_active_spells = active_spells.copy()
            next_active_spells[spells[i]] = spell_turns[i]

            # BOSS
            play_spells(nextme, nextboss, next_active_spells)
            decrement(next_active_spells, nextme)
            if nextboss[0] <= 0:
                if nextspent < minimum:
                    minimum = nextspent
                    print(minimum, played_spells + [spells[i]])
                return True

            damage(nextboss[1], nextme)
            play(nextme, nextboss, next_active_spells, nextspent, played_spells + [spells[i]])

play(me2.copy(), boss2.copy(), {}, 0, [])

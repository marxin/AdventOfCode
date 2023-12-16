#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;
use regex::{Captures, Regex};

/*

Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4

*/

#[derive(Debug, PartialEq, Clone)]
struct Unit {
    count: i64,
    hp: i64,
    weaknesses: Vec<String>,
    immune: Vec<String>,
    damage: i64,
    damage_type: String,
    initiative: i64,
}

impl Unit {
    fn new(captures: Captures) -> Self {
        let mut weaknesses = Vec::new();
        let mut immune = Vec::new();

        let parts = captures["details"].split("; ").collect_vec();
        for part in parts {
            if let Some(p) = part.strip_prefix("weak to ") {
                weaknesses = p.split(", ").map(|x| x.to_string()).collect();
            } else if let Some(p) = part.strip_prefix("immune to ") {
                immune = p.split(", ").map(|x| x.to_string()).collect();
            } else if part.is_empty() {
            } else {
                panic!("unexpected detail: {part}");
            }
        }

        Self {
            count: captures["units"].parse::<i64>().unwrap(),
            hp: captures["hp"].parse::<i64>().unwrap(),
            immune,
            weaknesses,
            damage: captures["damage"].parse::<i64>().unwrap(),
            damage_type: captures["damage_type"].to_string(),
            initiative: captures["init"].parse::<i64>().unwrap(),
        }
    }

    fn attack_selection(&self) -> (i64, i64) {
        (self.effective_power(), self.initiative)
    }

    fn effective_power(&self) -> i64 {
        self.count * self.damage
    }

    fn combat_damage(&self, other: &Unit) -> i64 {
        if other.immune.contains(&self.damage_type) {
            0
        } else if other.weaknesses.contains(&self.damage_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn target_selection(&self, other: &Unit) -> (i64, i64, i64) {
        (self.combat_damage(other), other.effective_power(), other.initiative)
    }
}

fn select_targets(source: &Vec<Unit>, target: &Vec<Unit>) -> HashMap<usize, usize> {
    let mut selection = HashMap::new();

    for src in source.iter().sorted_by(|a, b| a.attack_selection().cmp(&b.attack_selection())).rev() {
        let src_index = source.iter().position(|x| x == src).unwrap();

        let xxx = target.iter().sorted_by(|a, b| src.target_selection(a).cmp(&src.target_selection(b))).rev().collect_vec();    
        for dst in xxx {
            let dst_index = target.iter().position(|x| x == dst).unwrap();
            let damage = src.combat_damage(&dst);
            if damage > 0 && !selection.values().contains(&dst_index) {
                selection.insert(src_index, dst_index);
                break;
            }
        }
    }

    selection
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let re = Regex::new(r"(?<units>[0-9]+) units each with (?<hp>[0-9]+) hit points \((?<details>.*)?\) with an attack that does (?<damage>[0-9]+) (?<damage_type>.*) damage at initiative (?<init>[0-9]+)").unwrap();

    let parts = content.split("\n\n");

    let players = parts
        .map(|p| {
            p.lines()
                .skip(1)
                .map(|l| Unit::new(re.captures(l).unwrap()))
                .collect_vec()
        })
        .collect_vec();
    println!("{players:?}");

    let max_init = players.iter().map(|p| p.len()).sum::<usize>() as i64;
    println!("{max_init}");

    let mut immune = players[0].clone();
    let mut infection = players[1].clone();

    let mut round = 1;
    loop {
        println!("=== round #{round} ===");
        let immune_mapping = select_targets(&immune, &infection);
        let infection_mapping = select_targets(&infection, &immune);
        println!("{immune_mapping:?} {infection_mapping:?}");

        for i in (1..=max_init).rev() {
            if let Some(attacker) = immune.iter().position(|x| x.initiative == i) {
                if (immune_mapping.contains_key(&attacker)) {
                    let target =  infection.get_mut(immune_mapping[&attacker]).unwrap();
                    let attack = &immune[attacker];
                    let killed = attack.combat_damage(&target) / target.hp;
                    target.count -= killed;
                    println!("Immune #{} killed {killed} unit of #{}", attack.initiative, target.initiative);
                }
            }
            if let Some(attacker) = infection.iter().position(|x| x.initiative == i) {
                if infection_mapping.contains_key(&attacker) {
                    let target =  immune.get_mut(infection_mapping[&attacker]).unwrap();
                    let attack = &infection[attacker];
                    let killed = attack.combat_damage(&target) / target.hp;
                    target.count -= killed;
                    println!("Infection #{} killed {killed} unit of #{}", attack.initiative, target.initiative);
                }
            }
        }

        immune.retain(|x| x.count > 0);
        infection.retain(|x| x.count > 0);

        if immune.is_empty() || infection.is_empty() {
            break;
        }

        round += 1;
    }

    println!("IMMUNE = {immune:?}");
    println!("INFECTION = {infection:?}");

    println!("{} {}", immune.iter().map(|x| x.count).sum::<i64>(), infection.iter().map(|x| x.count).sum::<i64>());
}

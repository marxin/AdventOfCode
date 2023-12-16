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

#[derive(Debug)]
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
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();
    let re = Regex::new(r"(?<units>[0-9]+) units each with (?<hp>[0-9]+) hit points \((?<details>.*)\) with an attack that does (?<damage>[0-9]+) (?<damage_type>.*) damage at initiative (?<init>[0-9])+").unwrap();

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
}

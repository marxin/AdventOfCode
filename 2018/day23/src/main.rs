#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

use regex::Regex;

#[allow(unused)]
use itertools::Itertools;

// pos=<0,0,0>, r=4

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bot {
    pos: Vec<i32>,
    radius: i32,

    scaled_pos: Vec<i32>,
    scaled_radius: i32,
}

fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    a.iter().zip(b).map(|(&x, &y)| (x - y).abs()).sum()
}

fn middle(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    a.iter().zip(b).map(|(&x, &y)| (x + y) / 2).collect()
}

fn main() {
    let zero = vec![0, 0, 0];
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect_vec();

    assert_eq!(7, dist(&vec![1, 2, 3], &vec![2, 0, -1]));
    assert_eq!(vec![1, 1, 1], middle(&vec![1, 2, 3], &vec![2, 0, -1]));
    assert_eq!(0, dist(&vec![1, 2, 3], &vec![1, 2, 3]));

    let _width = lines.first().unwrap().len() as i32;
    let _height = lines.len() as i32;
    let mut bots = Vec::new();

    let pos_regex = Regex::new(r".*<(.*)>.*").unwrap();
    for (_y, line) in lines.iter().enumerate() {
        let pos = pos_regex.captures(line).unwrap()[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();
        let radius = line.split('=').last().unwrap().parse::<i32>().unwrap();
        bots.push(Bot {
            pos,
            radius,
            scaled_pos: Vec::new(),
            scaled_radius: -1,
        });
    }

    let mut scale = 2 << 21;

    for b in bots.iter_mut() {
        b.scaled_pos = b.pos.iter().map(|x| x / scale).collect_vec();
        b.scaled_radius = b.radius / scale + 2;
    }

    let mut min = bots[0].scaled_pos.clone();
    let mut max = bots[0].scaled_pos.clone();
    let mut radius = bots[0].scaled_radius;

    for b in bots.iter() {
        for d in 0..3 {
            min[d] = min[d].min(b.scaled_pos[d]);
            max[d] = max[d].max(b.scaled_pos[d]);
            radius = radius.max(b.scaled_radius);
        }
    }

    for d in 0..3 {
        min[d] -= radius + 2;
        max[d] += radius + 2;
    }

    let mut best_pos: Vec<i32> = Vec::new();
    let mut best;

    while scale != 0 {
        for b in bots.iter_mut() {
            b.scaled_pos = b.pos.iter().map(|x| x / scale).collect_vec();
            if scale == 1 {
                b.scaled_radius = b.radius;
            } else {
                b.scaled_radius = b.radius / scale + 2;
            }
            radius = radius.max(b.scaled_radius);
        }

        if !best_pos.is_empty() {
            best_pos = best_pos.iter().map(|x| 2 * x).collect();
            min = best_pos.iter().map(|x| x - 10).collect();
            max = best_pos.iter().map(|x| x + 10).collect();
        }

        println!("Scale={scale}: {min:?} - {max:?} {radius}");

        best = 0;
        for x in min[0]..=max[0] {
            for y in min[1]..=max[1] {
                for z in min[2]..=max[2] {
                    let pos = vec![x, y, z];
                    let count = bots
                        .iter()
                        .filter(|b| dist(&b.scaled_pos, &pos) <= b.scaled_radius)
                        .count();
                    if count >= best {
                        // println!("Better {count}");
                        if count == best {
                            if dist(&pos, &zero) < dist(&best_pos, &zero) {
                                best_pos = pos;
                            }
                        } else {
                            best = count;
                            best_pos = pos;
                        }
                    }
                }
            }
        }

        println!("Best: {best_pos:?} {best}",);
        scale /= 2;
    }

    println!("D={}", dist(&best_pos, &zero));

    /*
    let k = 20;
    let mut best_dist = dist(&best_pos, &zero);
    for x in -k..k {
        for y in -k..k {
            for z in -k..k {
                let pos = vec![best_pos[0] + x, best_pos[1] + y, best_pos[2] + z];
                let count = bots.iter().filter(|b| dist(&b.pos, &pos) <= b.radius).count();
                let d = dist(&pos, &zero);
                if count == best && d < best_dist {
                    best_dist = d;
                    println!("better dist={best_dist}");
                }
            }
        }
    }
    */
}

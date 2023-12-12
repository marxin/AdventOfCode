#[allow(unused)]
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, fs};

#[allow(unused)]
use itertools::Itertools;

#[allow(dead_code)]
const MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
struct Arrange
{
    pattern: String,
    pattern_groups: Vec<String>,
    groups: Vec<usize>,
}

fn get_groups(pattern: &str) -> Vec<usize> {
    pattern.split('.').filter(|g| g.len() > 0).map(|g| g.len()).collect_vec()
}

fn generate(todo: &[char], prefix: &String, results: & mut HashMap<Vec<usize>, usize>) {
    if !todo.is_empty() {
        match todo[0] {
            '?' => {
                let mut prefix = prefix.to_string();
                prefix.push('.');
                generate(&todo[1..], &prefix, results);
                prefix.pop();
                prefix.push('#');
                generate(&todo[1..], &prefix, results);
            },
            _ => {
                let mut prefix = prefix.to_string();
                prefix.push(todo[0]);
                generate(&todo[1..], &prefix, results);
            }
        }
    } else {
        results.entry(get_groups(&prefix)).and_modify(|x| *x += 1).or_insert(1);
    }
}

fn possibilites(pattern: &String) -> HashMap<Vec<usize>, usize> {
    let mut results = HashMap::new();
    let chars = pattern.chars().collect_vec();
    generate(&chars, &String::new(), &mut results);
    results
}
     
const TIMES: usize = 5;

// ???#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???"]
// [1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1] }
// assume only '?' and '#'

// ?????#???????????#?#??????#???????????#?#??????#???????????#?#??????#???????????#?#??????#???????????#?#
// [1, 4, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1]

#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheEntry {
    pattern: String,
    groups: VecDeque<usize>
}

fn combinations(pattern: &str, groups: & mut VecDeque<usize>, cache: & mut HashMap<CacheEntry, usize>) -> usize {
    if pattern.is_empty() && groups.is_empty() {
        return 1;
    }
    else if pattern.is_empty() {
        return 0;
    } else if groups.is_empty() {
        if pattern.chars().all(|x| x == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    let entry = CacheEntry {
        pattern: pattern.to_string(),
        groups: groups.clone()
    };

    if let Some(known) = cache.get(&entry) {
        return *known;
    }

    // println!("{pattern} {groups:?}");

    let mut times = 0;

    let n = groups.pop_front().unwrap();
    // start group now
    if pattern.len() >= n {
        let mut pattern = &pattern[n..];
        if !groups.is_empty() {
            if let Some(n) = pattern.chars().next() {
                if n == '?' {
                    pattern = &pattern[1..];
                    times += combinations(pattern, groups, cache);
                }
            }
        } else {
            times += combinations(pattern, groups, cache);
        }
    }
    groups.push_front(n);

    // or skip ? for now
    if pattern.chars().next().unwrap() == '?' {
        times += combinations(&pattern[1..], groups, cache);
    }

    cache.insert(entry, times);
    times
}

fn multi_groups(patterns: & mut Vec<String>, groups: &[usize], pos_cache: &HashMap<String, HashMap<Vec<usize>, usize>>) -> usize {
    if patterns.is_empty() && groups.is_empty() {
        return 1;
    } else if patterns.is_empty() {
        return 0;
    } else if groups.is_empty() {
        if patterns.iter().all(|x| !x.contains('#')) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut times = 0;

    let pattern = patterns.remove(0);
    for (start, c) in &pos_cache[&pattern] {
        if groups.starts_with(&start) {
            times += c * multi_groups(patterns, &groups[start.len()..], pos_cache);
        }
    }
    patterns.insert(0, pattern);

    times
}

fn main() {
    let mut arranges = Vec::new();

    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let parts = line.split_whitespace().collect_vec();

        let mut pattern = parts[0].to_string();
        pattern.push('?');
        pattern = pattern.repeat(TIMES);
        pattern.pop();

        arranges.push(Arrange {
            pattern: pattern.clone(),
            pattern_groups: pattern.split('.').filter(|x| x.len() > 0).map(|x| x.to_string()).collect_vec(),
            groups: parts[1].split(',').map(|x| x.parse().unwrap()).collect_vec().repeat(TIMES),
        });
    }

    // arranges.sort_by(|a, b| a.pattern_groups.len().cmp(&b.pattern_groups.len()));

    let s = String::from("?").repeat(20);
    let x = possibilites(&s);
    println!("{:?}", possibilites(&String::from("##?????#?")));

    let mut cache = HashMap::new();
    let r = combinations("???#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???????#?#?##?##???#???", & mut VecDeque::from(vec![1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1, 1, 1, 12, 1]), & mut cache);    
    println!("{}", r);


    let mut patter = "?###?????????".repeat(5);
    patter.pop();
    let r = combinations(&patter, & mut VecDeque::from(vec![3, 2, 1].repeat(5)), & mut cache);
    assert_eq!(r, 506250);

    let mut total = 0;
    for (i, a) in arranges.iter().enumerate() {
        if a.pattern_groups.len() == 1 {
            let mut cache = HashMap::new();
            println!("{a:?}");
            let com = combinations(&a.pattern, & mut VecDeque::from(a.groups.clone()), & mut cache);
            total += com;
            println!("#{i} = {com}");
        } else {
            println!("{a:?}");
            let mut possibilities = HashMap::new();
            for p in &a.pattern_groups {
                possibilities.insert(p.clone(), possibilites(p));
            }

            let r = multi_groups(& mut a.pattern_groups.clone(), &a.groups, &possibilities);
            println!("#{i} = {r}");
            total += r;
        }
    }

    println!("total={total}");
    
}

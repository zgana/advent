use std::{cmp::Ordering, fs};

use anyhow::Result;
use itertools::Itertools;

struct Rule((i32, i32));

/// Wrapper for rules of the form "<i0>|<i1>", e.g. "123|456".
impl Rule {
    fn new(line: &str) -> Self {
        let (s1, s2) = line.split('|').collect_tuple().unwrap();

        Self((s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap()))
    }
}

/// Parse rules and updates.
fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<i32>>) {
    let (rules, updates) = input.trim().split("\n\n").collect_tuple().unwrap();
    let rules = rules.split('\n').map(Rule::new).collect();
    let updates = updates
        .split('\n')
        .map(|s| s.split(',').map(|c| c.parse::<i32>().unwrap()).collect())
        .collect();

    (rules, updates)
}

/// Check if an update is "ordered" according to the Rules.
fn is_ordered(update: &[i32], rules: &Vec<Rule>) -> bool {
    for Rule((value0, value1)) in rules {
        if !(update.contains(value0) && update.contains(value1)) {
            continue;
        }

        let i0 = update.iter().position(|v| v == value0).unwrap();
        let i1 = update.iter().position(|v| v == value1).unwrap();
        if i0 > i1 {
            return false;
        }
    }

    true
}

/// Get the middle element in an odd-len array.
fn get_mid_elem(arr: &[i32]) -> i32 {
    let n = arr.len();
    arr[n / 2]
}

/// Facilitate sorting according to the Rules.
fn rules_cmp(a: i32, b: i32, rules: &Vec<Rule>) -> Ordering {
    for rule in rules {
        let Rule((value0, value1)) = rule;
        if (a, b) == (*value0, *value1) {
            return Ordering::Less;
        }
        if (a, b) == (*value1, *value0) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

/// Sum middle elements of ordered updates.
fn part1(updates: &[Vec<i32>], rules: &Vec<Rule>) -> i32 {
    let mut out = 0;
    for update in updates {
        if !is_ordered(update, rules) {
            continue;
        }

        out += get_mid_elem(update);
    }
    out
}

/// Sum middle elements of un-ordered updates, after ordering them.
fn part2(updates: &[Vec<i32>], rules: &Vec<Rule>) -> i32 {
    let mut out = 0;
    for update in updates {
        if is_ordered(update, rules) {
            continue;
        }

        let mut sorted = update.clone();
        sorted.sort_by(|a, b| rules_cmp(*a, *b, rules));

        out += get_mid_elem(&sorted);
    }

    out
}

// const TEST_INP: &str = "47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13
//
// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47";

fn main() -> Result<()> {
    let input = fs::read_to_string("input-05.txt")?;
    // let input = TEST_INP.to_string();
    let (rules, updates) = parse(&input);

    // dbg!(rules);
    // dbg!(updates);

    println!("{}", part1(&updates, &rules));
    println!("{}", part2(&updates, &rules));

    Ok(())
}

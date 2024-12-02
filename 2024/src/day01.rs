use itertools::Itertools;
use std::{collections::HashMap, fs, iter::zip};

use anyhow::Result;

/// Read input-01 into column Vecs.
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.trim().split('\n');
    let mut o1: Vec<i32> = Vec::new();
    let mut o2: Vec<i32> = Vec::new();
    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        o1.push(parts[0]);
        o2.push(parts[1]);
    }

    (o1, o2)
}

/// Solve part 1.
///
/// - Sort each Vec
/// - Sum abs deltas
fn part1(v1: &[i32], v2: &[i32]) -> i32 {
    let v1 = v1.iter().sorted();
    let v2 = v2.iter().sorted();
    let mut out = 0;

    for (i1, i2) in zip(v1, v2) {
        out += (i2 - i1).abs();
    }

    out
}

/// Solve part 2.
///
/// - Count v2 frequencies
/// - Sum v1 entries * v2 frequencies
fn part2(v1: &[i32], v2: &[i32]) -> i32 {
    let counts: &HashMap<i32, usize> = &v2.iter().copied().counts();

    let mut out = 0;
    for i in v1 {
        let mult = *counts.get(i).unwrap_or(&0) as i32;
        out += i * mult;
    }

    out
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-01.txt")?;
    let (v1, v2) = parse(&input);

    println!("{}", part1(&v1, &v2));
    println!("{}", part2(&v1, &v2));

    Ok(())
}

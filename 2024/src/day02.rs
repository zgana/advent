use std::fs;

use anyhow::Result;

/// Parse rows
fn parse(input: &str) -> Vec<Vec<i32>> {
    let lines = input.trim().split('\n');

    let mut out = Vec::new();
    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        out.push(parts);
    }

    out
}

/// Check if a row is valid
fn check_one(row: &[i32]) -> bool {
    let diff: Vec<i32> = row.windows(2).map(|s| s[1] - s[0]).collect();
    if diff.iter().all(|x| (1 <= *x) && (*x <= 3)) {
        return true;
    }
    if diff.iter().all(|x| (-3 <= *x) && (*x <= -1)) {
        return true;
    }

    false
}

/// Count valid rows
///
/// Nice and easy with functional pattern
fn part1(rows: &[Vec<i32>]) -> i32 {
    rows.iter().filter(|row: &&Vec<i32>| check_one(row)).count() as i32
}

/// Count valid rows, removing up to one problem entry before the check
///
/// Easier to write this one with loops and mutation
fn part2(rows: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    'main: for row in rows {
        if check_one(row) {
            count += 1;
            continue;
        }

        for i in 0..row.len() {
            let mut xrow = Vec::new();
            xrow.extend(&row[..i]);
            xrow.extend(&row[i + 1..row.len()]);
            if check_one(&xrow) {
                count += 1;
                continue 'main;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-02.txt")?;
    let rows = parse(&input);

    println!("{}", part1(&rows));
    println!("{}", part2(&rows));

    Ok(())
}

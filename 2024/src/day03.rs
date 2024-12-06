use std::fs;

use anyhow::Result;
use regex::Regex;

/// Get the sum of all mul() results
fn part1(text: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut out = 0i32;

    for (_, [value1, value2]) in re.captures_iter(text).map(|c| c.extract()) {
        out += value1.parse::<i32>().unwrap() * value2.parse::<i32>().unwrap();
    }

    out
}

/// Get the sum of all mul() results following do() (but not don't())
fn part2(text: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(?<toggle>do\(\)|don't\(\))").unwrap();
    let mut is_doing = true;
    let mut out = 0i32;

    for caps in re.captures_iter(text) {
        if let Some(instruction) = caps.name("toggle") {
            is_doing = instruction.as_str() == "do()";
            continue;
        }
        if !is_doing {
            continue;
        }
        let value1 = caps.get(1).unwrap().as_str();
        let value2 = caps.get(2).unwrap().as_str();
        out += value1.parse::<i32>().unwrap() * value2.parse::<i32>().unwrap();
    }

    out
}

fn main() -> Result<()> {
    let text = fs::read_to_string("input-03.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

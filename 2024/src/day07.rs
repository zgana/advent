use std::fs;

use anyhow::Result;
use itertools::Itertools;

/// Allowed operations (Concat only for part 2)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Mul,
    Add,
    Concat,
}

/// A result and the numbers that might lead you there
#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

impl Equation {
    /// Parse an equation from an input line
    fn from_text(text: &str) -> Self {
        let parts: Vec<&str> = text.trim().split(':').collect();
        let result = parts[0].parse().unwrap();
        let values = parts[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Self { result, values }
    }

    /// Determine whether any op sets make the equation valid
    fn is_maybe_valid(&self, ops: &[Op]) -> bool {
        let num_ops = self.values.len() - 1;
        let ops_generator = vec![ops; num_ops];
        for test_ops in ops_generator.into_iter().multi_cartesian_product() {
            if self.is_valid(&test_ops) {
                return true;
            }
        }

        false
    }

    /// Determine if a specific op set makes the equation valid
    fn is_valid(&self, ops: &[&Op]) -> bool {
        use Op::*;

        let mut tally = self.values[0];
        for (op, value) in ops.iter().zip(self.values.iter().skip(1)) {
            tally = match op {
                Add => tally + value,
                Mul => tally * value,
                Concat => format!("{}{}", tally, value).parse().unwrap(),
            };

            if tally > self.result {
                return false;
            }
        }

        tally == self.result
    }
}

/// Parse all input lines into Equations
fn parse(input: &str) -> Vec<Equation> {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    lines.iter().map(|line| Equation::from_text(line)).collect()
}

/// Sum the results of maybe-valid equations
fn sum_valid(eqs: &[Equation], ops: &[Op]) -> u64 {
    eqs.iter()
        .map(|eq| if eq.is_maybe_valid(ops) { eq.result } else { 0 })
        .sum()
}

/// Get sum_valid() for only + and *
fn part1(eqs: &[Equation]) -> u64 {
    let ops = vec![Op::Add, Op::Mul];

    sum_valid(eqs, &ops)
}

/// Get sum_valid() for + or * or ||
fn part2(eqs: &[Equation]) -> u64 {
    let ops = vec![Op::Add, Op::Mul, Op::Concat];

    sum_valid(eqs, &ops)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-07.txt")?;
    let eqs = parse(&input);

    println!("{}", part1(&eqs));
    println!("{}", part2(&eqs));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_TEXT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_one() {
        let text = "3267: 81 40 27";
        let result = 3267;
        let values = vec![81, 40, 27];
        let actual = Equation::from_text(text);
        let expected = Equation { result, values };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse() {
        let text = &TEST_TEXT;
        let _equations = parse(text);
        // dbg!(equations);
    }
}

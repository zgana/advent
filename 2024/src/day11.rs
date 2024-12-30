use std::{collections::HashMap, fs};

use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Array(Vec<u64>);

impl Array {
    fn new(text: &str) -> Self {
        Self(
            text.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect(),
        )
    }

    fn blink(&self) -> Self {
        let mut out = Vec::new();

        for value in self.0.iter() {
            // handle 0
            if *value == 0 {
                out.push(1);
                continue;
            }

            // handle even number of digits
            let str_value = value.to_string();
            let num_digits = str_value.len();
            if num_digits.rem_euclid(2) == 0 {
                let half_digits = num_digits.div_euclid(2);
                out.push(
                    str_value[0..half_digits]
                        .parse()
                        .expect("should be a number"),
                );
                out.push(
                    str_value[half_digits..num_digits]
                        .parse()
                        .expect("should be a number"),
                );
                continue;
            }

            // handle others
            out.push(value * 2024);
        }

        Self(out)
    }
}

impl ToString for Array {
    fn to_string(&self) -> String {
        self.0.iter().map(|value| value.to_string()).join(" ")
    }
}

#[derive(Clone, Debug)]
struct ArrayCounts(HashMap<u64, usize>);

impl ArrayCounts {
    fn new(text: &str) -> Self {
        let mut out = HashMap::new();

        for word in text.split_whitespace() {
            let num: u64 = word.parse().unwrap();
            *out.entry(num).or_default() += 1;
        }

        Self(out)
    }

    fn blink(&self) -> Self {
        let current = &self.0;
        let mut out = HashMap::new();

        for (num, count) in current.iter() {
            if *num == 0 {
                *out.entry(1).or_default() += count;
                continue;
            }

            let str_num = num.to_string();
            let num_digits = str_num.len();
            if num_digits.rem_euclid(2) == 0 {
                let half_digits = num_digits.div_euclid(2);
                let num1 = str_num[0..half_digits].parse().expect("should be a number");
                let num2 = str_num[half_digits..num_digits]
                    .parse()
                    .expect("should be a number");
                *out.entry(num1).or_default() += count;
                *out.entry(num2).or_default() += count;
                continue;
            }

            *out.entry(num * 2024).or_default() += count;
        }

        Self(out)
    }

    fn count(&self) -> usize {
        self.0.values().sum()
    }
}

fn part1(data: &Array) -> usize {
    let mut data: Array = data.clone();
    for _ in 0..25 {
        data = data.blink()
    }
    data.0.len()
}

fn part1b(data: &ArrayCounts) -> usize {
    let mut data: ArrayCounts = data.clone();
    for _ in 0..25 {
        data = data.blink()
    }
    data.count()
}

fn part2(data: &ArrayCounts) -> usize {
    let mut data: ArrayCounts = data.clone();
    for _ in 0..75 {
        data = data.blink()
    }
    data.count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-11.txt")?;
    let data1 = Array::new(&input);
    let data2 = ArrayCounts::new(&input);

    println!("{} (try2: {})", part1(&data1), part1b(&data2));
    println!("{}", part2(&data2));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST1: &str = "0 1 10 99 999";
    const TEST2: &str = "125 17";

    #[test]
    fn test_parse() {
        let a = Array::new(TEST1);
        let expected = Array(vec![0, 1, 10, 99, 999]);
        assert_eq!(a, expected);
    }

    #[test]
    fn test_parse_counts() {
        let a = ArrayCounts::new("0 0 1 1 2");
        let expected = HashMap::from([(0, 2), (1, 2), (2, 1)]);
        assert_eq!(a.0, expected);
    }

    #[test]
    fn test_blink_1() {
        let a = Array::new(TEST1);
        let expected = "1 2024 1 0 9 9 2021976";
        assert_eq!(a.blink().to_string(), expected);
    }

    #[test]
    fn test_blink_2_6() {
        let mut a = Array::new(TEST2);
        let expected = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
        for _ in 0..6 {
            a = a.blink()
        }
        assert_eq!(a.to_string(), expected);
    }

    #[test]
    fn test_blink_2_25() {
        let mut a = Array::new(TEST2);
        let expected = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
        for _ in 0..6 {
            a = a.blink()
        }
        assert_eq!(a.to_string(), expected);
    }

    #[test]
    fn test_part1() {
        let a = Array::new(TEST2);
        let expected = 55312;
        assert_eq!(part1(&a), expected);
    }

    // #[test]
    // fn test_part2() {
    //     todo!()
    // }

    //
}

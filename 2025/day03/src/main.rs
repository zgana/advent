use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let text = fs::read_to_string("data/03.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

/// Returns the part1 result
fn part1(text: &str) -> u64 {
    parse_banks(text).iter().map(|b| max_joltage(b, 2)).sum()
}

/// Returns the part2 result
fn part2(text: &str) -> u64 {
    parse_banks(text).iter().map(|b| max_joltage(b, 12)).sum()
}

type Bank = Vec<char>;

fn parse_bank(line: &str) -> Bank {
    line.chars().collect()
}

fn parse_banks(text: &str) -> Vec<Bank> {
    text.trim().split('\n').map(parse_bank).collect()
}

fn max_joltage_digits(bank: &Bank, num_digits: usize) -> Bank {
    if num_digits == 0 {
        return vec![];
    }

    let total_digits = bank.len();
    let search_digits = total_digits - num_digits + 1;

    let mut i_best = 0;
    let mut best = '\0';

    for (i, &c) in bank.iter().take(search_digits).enumerate() {
        if c > best {
            i_best = i;
            best = c;
        }
    }

    // now find the rest
    let more_digits = max_joltage_digits(&bank[i_best + 1..].to_vec(), num_digits - 1);
    let mut best_digits = vec![best];
    best_digits.extend(&more_digits);
    best_digits
    //
}

fn max_joltage(bank: &Bank, num_digits: usize) -> u64 {
    let digits = max_joltage_digits(bank, num_digits);
    let s: String = digits.iter().collect();
    s.parse()
        .unwrap_or_else(|_| panic!("could not parse \"{s}\""))
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_max_joltage_1() {
        let bank = parse_bank("987654321111111");
        let result = max_joltage(&bank, 2);
        assert_eq!(result, 98)
    }

    #[test]
    fn test_max_joltage_2() {
        let bank = parse_bank("811111111111119");
        let result = max_joltage(&bank, 2);
        assert_eq!(result, 89)
    }

    #[test]
    fn test_max_joltage_3() {
        let bank = parse_bank("234234234234278");
        let result = max_joltage(&bank, 2);
        assert_eq!(result, 78)
    }

    #[test]
    fn test_max_joltage_4() {
        let bank = parse_bank("818181911112111");
        let result = max_joltage(&bank, 2);
        assert_eq!(result, 92)
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST1);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_max_joltage_n_1() {
        let bank = parse_bank("987654321111111");
        let result = max_joltage(&bank, 12);
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_max_joltage_n_2() {
        let bank = parse_bank("811111111111119");
        let result = max_joltage(&bank, 12);
        assert_eq!(result, 811111111119)
    }

    #[test]
    fn test_max_joltage_n_3() {
        let bank = parse_bank("234234234234278");
        let result = max_joltage(&bank, 12);
        assert_eq!(result, 434234234278)
    }

    #[test]
    fn test_max_joltage_n_4() {
        let bank = parse_bank("818181911112111");
        let result = max_joltage(&bank, 12);
        assert_eq!(result, 888911112111)
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST1);
        assert_eq!(result, 3121910778619);
    }

    // ...
}

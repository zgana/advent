use anyhow::Result;
use std::{fs, ops::RangeInclusive};

/// Returns the part1 result
fn part1(text: &str) -> u64 {
    let mut result = 0;
    let ranges = parse_ranges(text);
    for range in ranges {
        for n in range {
            if !is_valid(n, false) {
                result += n;
            }
        }
    }

    result
}

/// Returns the part2 result
fn part2(text: &str) -> u64 {
    let mut result = 0;
    let ranges = parse_ranges(text);
    for range in ranges {
        for n in range {
            if !is_valid(n, true) {
                result += n;
            }
        }
    }

    result
}

/// A range of IDs
type IdRange = RangeInclusive<u64>;

/// Parse one range
fn parse_range(s: &str) -> IdRange {
    let (s1, s2) = s
        .split_once('-')
        .unwrap_or_else(|| panic!("could not parse {}", s));
    let i1: u64 = s1
        .parse()
        .unwrap_or_else(|_| panic!("could not parse {}", s1));
    let i2: u64 = s2
        .parse()
        .unwrap_or_else(|_| panic!("could not parse {}", s2));
    i1..=i2
}

/// Parse many ranges
fn parse_ranges(text: &str) -> Vec<IdRange> {
    text.trim().split(',').map(parse_range).collect()
}

/// Check whether an id is valid
fn is_valid(n: u64, multi: bool) -> bool {
    // let digits: Vec<_> = n.to_string().chars().collect();
    let digits = n.to_string();
    let num_digits = digits.len();

    // check for any number of repetitions
    'outer: for chunksize in 1..=num_digits / 2 {
        // need an even subdivision...
        if num_digits.rem_euclid(chunksize) != 0 {
            continue;
        }

        // `multi` is part2 behaviour
        // otherwise only check chunksize that is half num_digits
        if !multi && num_digits / chunksize != 2 {
            continue;
        }

        // split the string into chunks and check if they all match
        let chunks: Vec<_> = digits.as_bytes().chunks(chunksize).collect();
        let base = chunks.first().unwrap();
        for other in chunks.iter().skip(1) {
            if other != base {
                // if they don't match then this chunksize won't do
                continue 'outer;
            }
        }

        // if we got here then all chunks matched
        return false;
    }

    // if we got here then no chunksize proved it's in_valid
    true
}

fn main() -> Result<()> {
    let text = fs::read_to_string("data/02.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parse_range() {
        let s = "11-22";
        let r = parse_range(s);
        assert_eq!(*r.start(), 11);
        assert_eq!(*r.end(), 22);
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges(TEST1);
        let n = ranges.len();
        assert_eq!(ranges.len(), 11);
        assert_eq!(ranges[0], (11..=22));
        assert_eq!(ranges[n - 1], (2121212118..=2121212124));
    }

    #[test]
    fn test_is_valid_1() {
        for n in 11..=22 {
            let v = is_valid(n, false);
            assert_eq!(v, !(n == 11 || n == 22), "v = {}", v);
        }
    }

    #[test]
    fn test_is_valid_2() {
        for n in 95..=115 {
            let v = is_valid(n, false);
            assert_eq!(v, (n != 99), "v = {}", v);
        }
    }

    #[test]
    fn test_is_valid_3() {
        for n in 998..=1012 {
            let v = is_valid(n, false);
            assert_eq!(v, (n != 1010), "v = {}", v);
        }
    }

    #[test]
    fn test_is_valid_big1() {
        assert!(!is_valid(1188511885, false));
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST1);
        assert_eq!(result, 1227775554);
    }
    // ...
}

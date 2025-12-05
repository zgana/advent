use anyhow::Result;
use std::{
    cmp::{max, min},
    collections::VecDeque,
    fs,
    ops::RangeInclusive,
};

fn main() -> Result<()> {
    let text = fs::read_to_string("data/05.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

/// Returns the part1 result
fn part1(text: &str) -> u64 {
    let p = Problem::from_text(text);
    let mut out = 0;

    for id in p.ids {
        for r in p.ranges.iter() {
            if r.contains(&id) {
                out += 1;
                break;
            }
        }
    }

    out
}

/// Returns the part2 result
fn part2(text: &str) -> u64 {
    let p = Problem::from_text(text);
    let ranges = merge_ranges(p.ranges);
    let mut out = 0;

    for range in ranges {
        out += range.end() - range.start() + 1;
    }

    out
}

/// A range of "recipe" ids
type IDRange = RangeInclusive<u64>;

/// The problem components: range list and id list
#[derive(Debug)]
struct Problem {
    ranges: Vec<IDRange>,
    ids: Vec<u64>,
}

impl Problem {
    /// Creates a Problem from a text block.
    fn from_text(text: &str) -> Self {
        let (ranges_para, ids_para) = text
            .trim()
            .split_once("\n\n")
            .unwrap_or_else(|| panic!("could not parse text:\n{text}"));

        let ranges = ranges_para
            .trim()
            .split('\n')
            .map(|s| {
                let (a, b) = s
                    .split_once('-')
                    .unwrap_or_else(|| panic!("failed to split range '{s}'"));
                let a = a
                    .parse()
                    .unwrap_or_else(|_| panic!("failed to parse {a} as int"));
                let b = b
                    .parse()
                    .unwrap_or_else(|_| panic!("failed to parse {a} as int"));
                a..=b
            })
            .collect();

        let ids = ids_para
            .trim()
            .split('\n')
            .map(|s| s.parse().unwrap())
            .collect();

        Self { ranges, ids }
    }
}

/// Run the merging algorithm to completion
fn merge_ranges(ranges: Vec<IDRange>) -> Vec<IDRange> {
    let mut ranges = ranges;
    let maxiter = 10;
    for _ in 0..maxiter {
        let (these_ranges, num_merges) = merge_ranges_once(&ranges);
        if num_merges == 0 {
            return ranges.clone();
        }
        ranges = these_ranges;
    }
    panic!("did not complete merge in {maxiter} iterations");
}

/// Run a single pass of the range merging algorithm
fn merge_ranges_once(ranges: &[IDRange]) -> (Vec<IDRange>, usize) {
    let mut ranges: VecDeque<&IDRange> = ranges.iter().collect();
    let mut merged = Vec::new();
    let mut num_merges = 0;

    while !ranges.is_empty() {
        // split into first and the rest
        let mut range = ranges.pop_front().expect("could not pop_front").clone();
        let others = ranges.clone();

        // loop over others, keeping track of how many merges occur
        let mut this_num_merges = 0;
        for (i, &other) in others.iter().enumerate() {
            if ranges_overlap(&range, other) {
                // if they overlap, the first eats the other
                // index of other depends on how many times first already merged with others
                let removal_index = i - this_num_merges;
                ranges.remove(removal_index);
                range = coalesce_ranges(&range, other);
                this_num_merges += 1;
            }
            num_merges += this_num_merges;
        }
        merged.push(range.clone());
    }

    (merged, num_merges)
}

/// Whether two ranges overlap
fn ranges_overlap(r1: &IDRange, r2: &IDRange) -> bool {
    !((r1.start() > r2.end()) || (r2.start() > r1.end()))
}

/// Range that includes two given ranges
fn coalesce_ranges(r1: &IDRange, r2: &IDRange) -> IDRange {
    let mi = *min(r1.start(), r2.start());
    let ma = *max(r1.end(), r2.end());
    mi..=ma
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "

3-5
10-14
16-20
12-18

1
5
8
11
17
32

";

    #[test]
    fn test_problem() {
        let _p = Problem::from_text(TEST1);
        // dbg!(p);
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_ranges_overlap() {
        let r1 = 1..=3;
        let r2 = 3..=5;
        assert!(ranges_overlap(&r1, &r2))
    }

    #[test]
    fn test_merge_ranges() {
        let ranges = vec![1..=3, 2..=4, 3..=5, 7..=9, 8..=10];
        let ranges = merge_ranges(ranges);
        // dbg!(&ranges);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], 1..=5);
        assert_eq!(ranges[1], 7..=10);
    }

    #[test]
    fn test_part() {
        let result = part2(TEST1);
        assert_eq!(result, 14);
    }

    // ...
}

use std::{fmt::Display, fs, iter, ops::Range};

use anyhow::Result;
use itertools::Itertools;

/// Find all ranges of constant Some(value) in `data`.
fn value_ranges(data: &[Option<u64>]) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut scanned = 0usize;
    for value in 0..=data[data.len() - 1].expect("last value should be Some") {
        // search for value
        while data[scanned].is_none() || data[scanned].unwrap() < value {
            scanned += 1;
        }
        // note first appearance
        let i_start = scanned;
        // search for last appearance
        loop {
            scanned += 1;
            if scanned == data.len() || data[scanned].is_none() || data[scanned].unwrap() > value {
                break;
            }
        }
        // note last appearance
        let i_end = scanned;
        // record range
        out.push(i_start..i_end);
    }
    out
}

/// Find all ranges of None in `data`.
fn gap_ranges(data: &[Option<u64>]) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut i_start = 0;
    for ((_, v0), (i1, v1)) in data.iter().enumerate().zip(data.iter().enumerate().skip(1)) {
        if v0.is_some() && v1.is_none() {
            i_start = i1;
        }
        if v0.is_none() && v1.is_some() {
            let i_end = i1;
            out.push(i_start..i1);
        }
    }

    out
}

/// Representation of a disk.
struct Disk(Vec<Option<u64>>);

impl Disk {
    /// Parse disk from puzzle input.
    fn from_text(text: &str) -> Self {
        let sizes: Vec<_> = text
            .trim()
            .chars()
            .step_by(2)
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let gaps: Vec<_> = text
            .trim()
            .chars()
            .skip(1)
            .step_by(2)
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let len = sizes.iter().sum::<usize>() + gaps.iter().sum::<usize>();
        let mut data = vec![None; len];
        let mut index = 0usize;
        for (i_file, (size, gap)) in sizes
            .iter()
            .zip(gaps.iter().chain(iter::once(&0usize)))
            .enumerate()
        {
            for slot in data.iter_mut().skip(index).take(*size) {
                *slot = Some(i_file as u64);
            }
            index += size + gap;
        }

        Self(data)
    }

    /// Access the data.
    fn data(&self) -> &Vec<Option<u64>> {
        &self.0
    }

    /// Calculate puzzle output.
    fn checksum(&self) -> u64 {
        self.data()
            .iter()
            .enumerate()
            .map(|(i, x)| i as u64 * x.unwrap_or(0))
            .sum()
    }

    /// Write disk in short form, as in puzzle description.
    #[allow(dead_code)]
    fn to_brief(&self) -> String {
        let mut out = vec!['.'; self.0.len()];
        for (i, c) in self.data().iter().enumerate() {
            if let Some(c) = *c {
                out[i] = char::from_u32(c as u32 + '0' as u32).unwrap();
            }
        }
        let out = String::from_iter(out.iter());
        out
    }

    /// Compress the disk as in part 1.
    fn compress(&self) -> Self {
        let mut data = self.data().clone();
        let i_dest = self.data().iter().positions(|&v| v.is_none());
        let i_src = self.data().iter().positions(|&v| v.is_some()).rev();
        for (i_dest, i_src) in i_dest.zip(i_src) {
            if i_dest >= i_src {
                break;
            }
            data[i_dest] = data[i_src];
            data[i_src] = None;
        }
        Self(data)
    }

    /// Compress the disk in defragmented way, as in part 2.
    fn compress_defrag(&self) -> Self {
        let mut data = self.data().clone();
        for range in value_ranges(self.data()).iter().rev() {
            let need = range.len();
            // NOTE(perf): we should not need to recompute gap_ranges() in its entirety every
            // iteration.  would be more efficient to recompute only ranges that have changed
            for gap_range in gap_ranges(&data) {
                if gap_range.end > range.start {
                    break;
                }
                let avail = gap_range.len();
                if need <= avail {
                    for (i_src, i_dest) in range.clone().zip(gap_range.clone()) {
                        data[i_dest] = data[i_src];
                        data[i_src] = None;
                    }
                }
            }
        }

        Self(data)
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Disk(")?;
        for (i, value) in self.0.iter().enumerate() {
            let s = if let Some(x) = value {
                format!("{}", x)
            } else {
                "---".to_string()
            };
            writeln!(f, "{:9}: {:3}", i, s)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

fn part1(disk: &Disk) -> u64 {
    disk.compress().checksum()
}

fn part2(disk: &Disk) -> u64 {
    disk.compress_defrag().checksum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-09.txt")?;
    let data = Disk::from_text(&input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_TEXT: &str = "2333133121414131402";
    const TEST_TEXT2: &str = "12345";

    #[test]
    fn test_parse() {
        let disk = Disk::from_text(TEST_TEXT2);

        let expected: Vec<Option<u64>> = vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ];

        assert_eq!(disk.data(), &expected);
    }

    #[test]
    fn test_compress() {
        let disk = Disk::from_text(TEST_TEXT2);
        assert_eq!(disk.compress().to_brief(), "022111222......");

        let disk = Disk::from_text(TEST_TEXT).compress();
        assert_eq!(
            disk.to_brief(),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn test_ranges() {
        let disk = Disk::from_text(TEST_TEXT2);
        let expected = vec![0..1, 3..6, 10..15];
        assert_eq!(value_ranges(disk.data()), expected);
    }

    #[test]
    fn test_gap_ranges() {
        let disk = Disk::from_text(TEST_TEXT2);
        let expected = vec![1..3, 6..10];
        assert_eq!(gap_ranges(disk.data()), expected);
    }

    #[test]
    fn test_compress_defrag() {
        let disk = Disk::from_text(TEST_TEXT).compress_defrag();
        assert_eq!(
            disk.to_brief(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn test_part1() {
        let disk = Disk::from_text(TEST_TEXT).compress();
        assert_eq!(disk.checksum(), 1928);
    }

    #[test]
    fn test_part2() {
        let disk = Disk::from_text(TEST_TEXT).compress_defrag();
        assert_eq!(disk.checksum(), 2858);
    }
}

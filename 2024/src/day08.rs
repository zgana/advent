use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt, fs,
};

use anyhow::Result;
use itertools::Itertools;
use nalgebra::DMatrix;
use num::Integer;

/// A location (row, col)
#[derive(Clone, Eq, Hash, PartialEq)]
struct Loc(i32, i32);

impl Loc {
    /// The row
    fn row(&self) -> i32 {
        self.0
    }

    /// The row as usize
    fn row_usize(&self) -> usize {
        self.0 as usize
    }

    /// The col
    fn col(&self) -> i32 {
        self.1
    }

    /// The col as usize
    fn col_usize(&self) -> usize {
        self.1 as usize
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Loc({}, {})", self.0, self.1)
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// A map of the territory
#[derive(Clone)]
struct Map(DMatrix<char>);

impl Map {
    /// Make a map from an input string like in the input file
    fn from_text(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        let nrow = lines.len();
        let ncol = lines[0].len();

        let mat = DMatrix::from_fn(nrow, ncol, |row, col| {
            lines[row].chars().collect::<Vec<_>>()[col]
        });

        Self(mat)
    }

    fn nrows(&self) -> i32 {
        self.0.nrows() as i32
    }
    fn ncols(&self) -> i32 {
        self.0.ncols() as i32
    }

    /// Iterater over the locs
    fn locs(&self) -> impl Iterator<Item = Loc> {
        (0..self.nrows())
            .cartesian_product(0..self.ncols())
            .map(|(r, c)| Loc(r, c))
    }

    /// Find "frequencies"
    fn uniq_freqs(&self) -> Vec<char> {
        let mut freqs = Vec::new();
        for loc in self.locs() {
            let c = self.0[(loc.row_usize(), loc.col_usize())];
            if c != '.' && !freqs.contains(&c) {
                freqs.push(c);
            }
        }
        freqs
    }

    /// Find frequency locations
    fn freq_locs(&self) -> HashMap<char, Vec<Loc>> {
        let mut out = HashMap::new();
        for freq in self.uniq_freqs() {
            let mut locs = Vec::new();
            for loc in self.locs() {
                if self.0[(loc.row_usize(), loc.col_usize())] == freq {
                    locs.push(loc.clone())
                }
            }
            out.insert(freq, locs);
        }

        out
    }

    /// Check bounds
    fn is_inbounds(&self, loc: &Loc) -> bool {
        let nrows = self.0.nrows() as i32;
        let ncols = self.0.ncols() as i32;
        0 <= loc.0 && loc.0 < nrows && 0 <= loc.1 && loc.1 < ncols
    }
}

fn part1(data: &Map) -> usize {
    let mut antinodes: HashSet<Loc> = HashSet::new();
    for (_freq, locs) in data.freq_locs() {
        for nodes in locs.iter().combinations(2) {
            let node0 = nodes[0];
            let node1 = nodes[1];
            let drow = node1.row() - node0.row();
            let dcol = node1.col() - node0.col();
            let an0 = Loc(node0.row() - drow, node0.col() - dcol);
            let an1 = Loc(node1.row() + drow, node1.col() + dcol);
            if data.is_inbounds(&an0) {
                antinodes.insert(an0);
            }
            if data.is_inbounds(&an1) {
                antinodes.insert(an1);
            }
        }
    }

    antinodes.len()
}

fn part2(data: &Map) -> usize {
    let max_iter = cmp::max(data.nrows(), data.ncols());
    let mut antinodes: HashSet<Loc> = HashSet::new();
    for (_freq, locs) in data.freq_locs() {
        for nodes in locs.iter().combinations(2) {
            let node0 = nodes[0];
            let node1 = nodes[1];
            let drow = node1.row() - node0.row();
            let dcol = node1.col() - node0.col();
            let gcd = drow.gcd(&dcol);
            let drow = drow.div_euclid(gcd);
            let dcol = dcol.div_euclid(gcd);
            for sign in [-1, 1] {
                for i in 0..max_iter {
                    let an = Loc(node0.row() + sign * i * drow, node0.col() + sign * i * dcol);
                    if data.is_inbounds(&an) {
                        antinodes.insert(an);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-08.txt")?;
    let data = Map::from_text(&input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_TEXT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse() {
        let data = Map::from_text(TEST_TEXT);
        (&data.uniq_freqs());
        dbg!(data.freq_locs());
    }

    #[test]
    fn test_bounds() {
        let data = Map::from_text(TEST_TEXT);
        // println!("data.shape = {}, {}", data.0.nrows(), data.0.ncols());
        assert!(data.is_inbounds(&Loc(0, 0)));
        assert!(data.is_inbounds(&Loc(11, 11)));
        assert!(!data.is_inbounds(&Loc(12, 11)));
        assert!(!data.is_inbounds(&Loc(11, 12)));
        assert!(!data.is_inbounds(&Loc(-1, 0)));
        assert!(!data.is_inbounds(&Loc(0, -1)));
    }

    #[test]
    fn test_part1() {
        let data = Map::from_text(TEST_TEXT);
        assert_eq!(part1(&data), 14)
    }

    #[test]
    fn test_part2() {
        let data = Map::from_text(TEST_TEXT);
        assert_eq!(part2(&data), 34)
    }
}

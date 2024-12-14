use std::{
    collections::{HashMap, HashSet},
    fmt, fs, ops,
};

use anyhow::Result;
use itertools::Itertools;
use nalgebra::DMatrix;

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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

impl ops::Add<&Step> for Loc {
    type Output = Loc;

    fn add(self, step: &Step) -> Self {
        use Step::*;

        match step {
            Up => Self(self.0 - 1, self.1),
            Down => Self(self.0 + 1, self.1),
            Left => Self(self.0, self.1 - 1),
            Right => Self(self.0, self.1 + 1),
        }
    }
}

/// A map of the territory
#[derive(Clone)]
struct Map(DMatrix<i32>);

impl Map {
    /// Make a map from an input string like in the input file
    fn from_text(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().split('\n').collect();
        let nrow = lines.len();
        let ncol = lines[0].len();

        let mat = DMatrix::from_fn(nrow, ncol, |row, col| {
            lines[row]
                .chars()
                .map(|c| {
                    if let Some(d) = c.to_digit(10) {
                        d as i32
                    } else {
                        -1
                    }
                })
                .collect::<Vec<_>>()[col]
        });

        Self(mat)
    }

    /// Get the value at a Loc
    fn at(&self, loc: &Loc) -> i32 {
        if !self.is_inbounds(loc) {
            return -1;
        }

        let Loc(row, col) = loc;
        self.0[(*row as usize, *col as usize)]
    }

    /// Number of rows
    fn nrows(&self) -> i32 {
        self.0.nrows() as i32
    }

    /// Number of columns
    fn ncols(&self) -> i32 {
        self.0.ncols() as i32
    }

    /// Iterater over the locs
    fn locs(&self) -> impl Iterator<Item = Loc> {
        (0..self.nrows())
            .cartesian_product(0..self.ncols())
            .map(|(r, c)| Loc(r, c))
    }

    /// Check bounds
    fn is_inbounds(&self, loc: &Loc) -> bool {
        let nrows = self.0.nrows() as i32;
        let ncols = self.0.ncols() as i32;
        0 <= loc.0 && loc.0 < nrows && 0 <= loc.1 && loc.1 < ncols
    }

    /// Count number of 0-9 trails starting from a point
    fn find_tails(&self, loc: &Loc) -> HashMap<Loc, usize> {
        use Step::*;

        let current = self.at(loc);
        if current == -1 {
            // TODO: this is a bit sus but the tests and puzzles pass
            return HashMap::new();
        }

        if current == 9 {
            let mut out = HashMap::new();
            out.insert(loc.clone(), 1);
            return out;
        }

        let mut out = HashMap::new();
        for step in [Up, Down, Left, Right] {
            let next_loc = loc.clone() + &step;
            if self.at(&next_loc) == current + 1 {
                for (loc, count) in self.find_tails(&next_loc).iter() {
                    *out.entry(loc.clone()).or_default() += count;
                }
            }
        }
        out
    }
}

fn part1(data: &Map) -> usize {
    data.locs()
        .filter(|loc| data.at(loc) == 0)
        .map(|loc| data.find_tails(&loc).len())
        .sum()
}

fn part2(data: &Map) -> usize {
    data.locs()
        .filter(|loc| data.at(loc) == 0)
        .map(|loc| data.find_tails(&loc).values().sum::<usize>())
        .sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-10.txt")?;
    let data = Map::from_text(&input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST1: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const TEST2: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const TEST3: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

    const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_parse() {
        let m1 = Map::from_text(TEST1);
        let m2 = Map::from_text(TEST2);
        let m3 = Map::from_text(TEST3);
        let m = Map::from_text(TEST);
    }

    #[test]
    fn test_find_tails_1() {
        let m = Map::from_text(TEST1);
        let start = Loc(0, 3);
        let num_tails = m.find_tails(&start).len();
        assert_eq!(num_tails, 2);
    }

    #[test]
    fn test_find_tails_2() {
        let m = Map::from_text(TEST2);
        let start = Loc(0, 3);
        let num_tails = m.find_tails(&start).len();
        assert_eq!(num_tails, 4);
    }

    #[test]
    fn test_find_tails_3() {
        let m = Map::from_text(TEST3);
        let start = Loc(0, 1);
        let num_tails = m.find_tails(&start).len();
        assert_eq!(num_tails, 1);

        let start = Loc(6, 5);
        let num_tails = m.find_tails(&start).len();
        assert_eq!(num_tails, 2);
    }

    #[test]
    fn test_part1() {
        let m = Map::from_text(TEST);
        assert_eq!(part1(&m), 36);
    }

    #[test]
    fn test_part2() {
        let m = Map::from_text(TEST);
        assert_eq!(part2(&m), 81);
    }
}

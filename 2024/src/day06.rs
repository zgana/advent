use core::fmt;
use std::{collections::HashSet, fs, ops::Index};

use anyhow::Result;
use nalgebra::DMatrix;

/// Directions we could be going
#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    /// Next direction (right turns only)
    fn next(&self) -> Self {
        use Dir::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// Coordinate delta for each direction
    fn delta(&self) -> (i32, i32) {
        use Dir::*;
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    /// Char representation of a dir on the map (probably don't need this)
    fn value(&self) -> char {
        use Dir::*;
        match self {
            Up => '^',
            Right => '>',
            Down => 'v',
            Left => '<',
        }
    }
}

/// A location (row, col)
#[derive(Clone, Eq, Hash, PartialEq)]
struct Loc(i32, i32);

impl Loc {
    /// Next coordinates one step in a given direction
    fn next(&self, dir: Dir) -> Self {
        let Loc(x, y) = self;
        let (dx, dy) = dir.delta();
        Loc(x + dx, y + dy)
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

/// A trajectory: location plus direction
#[derive(Clone, Debug, PartialEq)]
struct Traj {
    loc: Loc,
    dir: Dir,
}

impl Traj {
    fn new(loc: &Loc, dir: Dir) -> Self {
        Self {
            loc: loc.clone(),
            dir,
        }
    }
}

/// A map of the territory
#[derive(Clone)]
struct Map(DMatrix<char>);

impl Map {
    /// Make a map from an input string like in the input file
    fn new(input: &str) -> Self {
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

    /// Update a cell on the map (probably don't need this)
    fn update(&mut self, loc: &Loc, value: char) {
        let Loc(row, col) = *loc;
        self.0[(row as usize, col as usize)] = value;
    }

    /// Determine the starting point (there should be exactly one '^')
    fn start_loc(&self) -> Loc {
        for row in 0..self.0.nrows() {
            for col in 0..self.0.ncols() {
                if self.0[(row, col)] == '^' {
                    return Loc(row as i32, col as i32);
                }
            }
        }
        panic!("should have found a starting location")
    }

    /// Check if a Loc is in bounds on the map
    fn contains(&self, loc: &Loc) -> bool {
        let Loc(row, col) = *loc;

        0 <= row && row < self.nrows() && 0 <= col && col < self.ncols()
    }

    /// Count visited locations (could've counted locations instead of map footprints...)
    fn count_visited(&self) -> i32 {
        let mut out = 0;

        for row in 0..self.0.nrows() {
            for col in 0..self.0.ncols() {
                if ['^', '>', 'v', '<'].contains(&self.0[(row, col)]) {
                    out += 1;
                }
            }
        }

        out
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.0.nrows() {
            for col in 0..self.0.ncols() {
                write!(f, "{}", self.0[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self)?;
        Ok(())
    }
}

impl Index<&Loc> for Map {
    type Output = char;

    fn index(&self, loc: &Loc) -> &char {
        let Loc(x, y) = loc;
        &self.0[(*x as usize, *y as usize)]
    }
}

/// Possible outcomes: exit the map or get stuck in a loop
#[derive(Debug, PartialEq)]
enum Outcome {
    Exit,
    Loop,
}

/// Path to track where we've been
#[derive(Debug)]
struct Path {
    _init_state: Map,
    final_state: Map,
    history: Vec<Traj>,
    outcome: Outcome,
}

impl Path {
    /// Generate a Path by following the rules starting from some location
    fn generate(init_state: Map, loc: &Loc, dir: Dir) -> Self {
        let mut state = init_state.clone();
        let mut loc = loc.clone();
        let mut dir = dir;
        let mut history = vec![Traj::new(&loc, dir)];
        let mut corners = vec![];
        let mut outcome = None;

        let max_iter = 10_000;
        for _ in 0..max_iter {
            let next_loc = loc.next(dir);

            // handle exit
            if !state.contains(&next_loc) {
                outcome = Some(Outcome::Exit);
                break;
            }

            // take action
            if state[&next_loc] == '#' {
                // turn due to obstruction
                dir = dir.next();
                if dir == Dir::Up {
                    // check for loops any time we turn from Left to Up
                    if corners.contains(&loc) {
                        outcome = Some(Outcome::Loop);
                        break;
                    }
                    corners.push(loc.clone())
                }
            } else {
                // take a step in current direction
                loc = next_loc;
            }

            let traj = Traj::new(&loc, dir);

            state.update(&loc, dir.value());
            history.push(traj);
        }

        Self {
            _init_state: init_state,
            final_state: state,
            history,
            outcome: outcome.unwrap(),
        }
    }
}

// Calculate the path and count visited spots.
fn part1(input: &str) -> i32 {
    let layout = Map::new(input);
    let loc = layout.start_loc();
    let path = Path::generate(layout, &loc, Dir::Up);
    assert!(path.outcome == Outcome::Exit);
    path.final_state.count_visited()
}

// Calculate the path, and try obstructing at each distinct point on that path.
fn part2(input: &str) -> i32 {
    // run the ordinary routine
    let layout = Map::new(input);
    let loc = layout.start_loc();
    let path = Path::generate(layout.clone(), &loc, Dir::Up);
    assert!(path.outcome == Outcome::Exit);

    // now try throwing an obstacle in front, each step of the way, and see if it results in a loop
    let mut obstruct_locs = HashSet::new();
    for traj in path.history.iter().skip(1) {
        let obstruct_loc = &traj.loc;
        // don't retry an obstruction
        if obstruct_locs.contains(obstruct_loc) {
            continue;
        }

        // block the space
        let mut testlayout = layout.clone();
        testlayout.update(obstruct_loc, '#');
        let testpath = Path::generate(testlayout.clone(), &loc, Dir::Up);

        // note any loops
        if testpath.outcome == Outcome::Loop {
            obstruct_locs.insert(obstruct_loc);
        }
    }

    obstruct_locs.len() as i32
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-06.txt")?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_layout() {
        let _layout = Map::new(TEST_INPUT);
        // dbg!(layout);
    }

    #[test]
    fn test_part1() {
        let v1 = part1(TEST_INPUT);
        assert_eq!(v1, 41)
    }

    #[test]
    fn test_part2() {
        let v2 = part2(TEST_INPUT);
        assert_eq!(v2, 6)
    }
}

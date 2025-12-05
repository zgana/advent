use anyhow::Result;
use std::{fmt, fs};

/// A move: rotating the dial left or right
#[derive(Debug, Clone, PartialEq)]
struct Move(i32);

impl Move {
    /// Creates a Move based on an input line.
    fn from_line(line: &str) -> Self {
        let dir = line.chars().nth(0).expect("could not read dir from line");
        let n: i32 = line[1..].parse().expect("could not parse n from line");
        match dir {
            'L' => Move(-n),
            'R' => Move(n),
            _ => panic!("could not parse line:\n{}", n),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = if self.0 >= 0 { 'R' } else { 'L' };
        f.pad(&format!("{}{}", dir, self.0.abs()))
    }
}

/// A position: a dial state that can be normalized to the range [0, 99]
#[derive(Debug, Clone)]
struct Position(i32);

impl Position {
    /// Creates a new Position by applying a Move
    fn apply(&self, m: &Move) -> Position {
        Position(self.0 + m.0)
    }

    /// Creates a new Position by applying a Move while tracking clicks at 0
    fn apply_counted(&self, m: &Move) -> (Position, u32) {
        let pos0 = self.0;
        let pos1 = self.0 + m.0;

        // we're going to do roughly (a//100 - b//100).abs()
        // the following switcheroo avoids incorrect behavior
        // when moving left from exact multiples of 100
        // TODO: there is probably a better way
        let (a, b) = if m.0 >= 0 {
            (pos1, pos0)
        } else {
            (-pos0, -pos1)
        };
        let delta_zeros = (a.div_euclid(100) - b.div_euclid(100)).unsigned_abs();

        (Position(pos1), delta_zeros)
    }

    /// The position normalized to the range [0, 99]
    fn pos(&self) -> i32 {
        self.0.rem_euclid(100)
    }
}

/// Reads input text into a `Vec<Move>`
fn parse_moves(text: &str) -> Vec<Move> {
    text.trim().split('\n').map(Move::from_line).collect()
}

/// Returns the part1 result
fn part1(moves: &Vec<Move>) -> u32 {
    let mut position = Position(50);
    let mut num_zeros = 0;

    for m in moves {
        position = position.apply(m);
        if position.pos() == 0 {
            num_zeros += 1;
        }
    }

    num_zeros
}

/// Returns the part2 result
fn part2(moves: &Vec<Move>) -> u32 {
    let mut position = Position(50);
    let mut num_zeros = 0;

    for m in moves {
        let (new_position, n) = position.apply_counted(m);
        position = new_position;
        num_zeros += n;
    }

    num_zeros
}

fn main() -> Result<()> {
    let input = fs::read_to_string("data/01.txt")?;
    let moves = parse_moves(&input);

    println!("{}", part1(&moves));
    println!("{}", part2(&moves));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_move() {
        let mut p = Position(50);
        p = p.apply(&Move(-20)).apply(&Move(10));
        assert_eq!(p.0, 40);
    }

    #[test]
    fn test_move_counted_1() {
        let p = Position(50);
        let (p, n) = p.apply_counted(&Move(-68));
        assert_eq!(p.pos(), 82);
        assert_eq!(n, 1);
    }

    #[test]
    fn test_move_counted_2() {
        let p = Position(50);
        let (p, n) = p.apply_counted(&Move(1000));
        assert_eq!(p.pos(), 50);
        assert_eq!(n, 10);
    }

    #[test]
    fn test_move_counted_3() {
        let p = Position(50);
        let (p, n) = p.apply_counted(&Move(-1000));
        assert_eq!(p.pos(), 50);
        assert_eq!(n, 10);
    }

    #[test]
    fn test_move_counted_several() {
        let mut pos = Position(50);
        let moves = parse_moves(TEST1);
        let ps = vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32];
        let ns = vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1];
        for ((m, &p), &c) in moves.iter().zip(ps.iter()).zip(ns.iter()) {
            let (newpos, n) = pos.apply_counted(m);
            pos = newpos;
            println!(
                "MOVE {:>8} || pos = {} ({}) vs {} // n = {} vs {}",
                m,
                pos.pos(),
                pos.0,
                p,
                n,
                c
            );
            assert_eq!(pos.pos(), p);
            assert_eq!(n, c as u32);
        }
    }

    #[test]
    fn test_parse_moves_1() {
        let s = "L68";
        let moves = parse_moves(s);

        let expected = vec![Move(-68)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_parse_moves_2() {
        let s = "L68\nL30\nR48";
        let moves = parse_moves(s);

        let expected = vec![Move(-68), Move(-30), Move(48)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_part1() {
        let moves = parse_moves(TEST1);
        let expected = 3;
        let result = part1(&moves);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_part2() {
        let moves = parse_moves(TEST1);
        let expected = 6;
        let result = part2(&moves);
        assert_eq!(result, expected)
    }
}

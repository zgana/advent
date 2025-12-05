use std::{collections::VecDeque, fs};

use anyhow::Result;
use itertools::Itertools;
use mdraoc::grid::{Delta, Grid, Loc};

fn inflate(grid: &Grid<char>) -> Grid<char> {
    let mut out = Grid::new(grid.nrows(), grid.ncols() * 2);

    for (loc, val) in grid.each_item() {
        let dest1 = Loc(loc.row(), 2 * loc.col());
        let dest2 = dest1 + Delta::RIGHT;
        if *val == 'O' {
            *out.at_mut(dest1) = '[';
            *out.at_mut(dest2) = ']';
        } else if *val == '@' {
            *out.at_mut(dest1) = *val;
            *out.at_mut(dest2) = '.';
        } else {
            *out.at_mut(dest1) = *val;
            *out.at_mut(dest2) = *val;
        }
    }

    out
}

fn parse(input: &str) -> (Grid<char>, String) {
    let (grid_text, steps) = input
        .split("\n\n")
        .collect_tuple()
        .expect("should find two paragraphs");
    let grid = Grid::parse_char_grid(grid_text);
    let steps = steps.replace('\n', "").to_owned();

    (grid, steps)
}

fn try_step(grid: &Grid<char>, step: char) -> Grid<char> {
    let loc = grid
        .each_item()
        .filter(|(_loc, val)| **val == '@')
        .map(|(loc, _val)| loc)
        .collect::<Vec<_>>()[0];
    let delta = match step {
        '^' => Delta::UP,
        'v' => Delta::DOWN,
        '<' => Delta::LEFT,
        '>' => Delta::RIGHT,
        _ => panic!("step should be one of <^>v"),
    };

    // println!("loc = {} step = {} delta = {}", loc, step, delta);
    // println!("grid = \n{}", grid);

    // find what moves
    let mut to_move = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(loc);
    while !queue.is_empty() {
        // handle this one
        let loc = queue
            .pop_front()
            .expect("should have guaranteed !queue.is_empty()");
        if *grid.at(loc) == '#' {
            return grid.clone();
        }
        to_move.push(loc);

        // handle next
        let next_loc = loc + delta;
        if *grid.at(next_loc) == '[' {
            queue.push_back(next_loc);
            let other = next_loc + Delta::RIGHT;
            if other != loc {
                queue.push_back(next_loc + Delta::RIGHT);
            }
        }
        if *grid.at(next_loc) == ']' {
            queue.push_back(next_loc);
            let other = next_loc + Delta::LEFT;
            if other != loc {
                queue.push_back(next_loc + Delta::LEFT);
            }
        }
        if *grid.at(next_loc) == 'O' {
            queue.push_back(next_loc);
        }
    }

    // move them
    let mut out = grid.clone();
    for loc in to_move.iter().rev().cloned() {
        let next_loc = loc + delta;
        if *out.at(next_loc) == '#' {
            return grid.clone();
        }
        *out.at_mut(next_loc) = *grid.at(loc);
        *out.at_mut(loc) = '.';
    }

    out
}

fn sum_gps(grid: &Grid<char>) -> u32 {
    grid.each_item()
        .filter(|(_loc, val)| **val == 'O' || **val == '[')
        .map(|(loc, _val)| loc.row_u32() * 100 + loc.col_u32())
        .sum()
}

fn part1(input: &str) -> u32 {
    let (mut grid, steps) = parse(input);
    println!("{}", grid);
    for step in steps.chars() {
        grid = try_step(&grid, step);
    }
    println!("{}", grid);
    sum_gps(&grid)
}

fn part2(input: &str) -> u32 {
    let (grid, steps) = parse(input);
    let mut grid = inflate(&grid);
    println!("{}", grid);
    for step in steps.chars() {
        grid = try_step(&grid, step);
    }
    println!("{}", grid);
    sum_gps(&grid)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-15.txt")?;
    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_parse() {
        println!();
        let (grid, _steps) = parse(TEST);
        let grid = inflate(&grid);
        println!("\n\n{}", &grid);
    }

    #[test]
    fn test_part1_small() {
        println!();
        let result = part1(TEST);
        let expected = 2028;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_large() {
        println!();
        let result = part1(TEST2);
        let expected = 10092;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        println!();
        let result = part2(TEST2);
        let expected = 9021;
        assert_eq!(result, expected);
    }
}

use anyhow::Result;
use std::fs;

type Grid = grid::Grid<char>;

fn main() -> Result<()> {
    let text = fs::read_to_string("data/04.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

/// Returns the part1 result
fn part1(text: &str) -> u64 {
    let g = Grid::new_char(text).unwrap();

    let mut out = 0;
    for row in 0..g.height() {
        for col in 0..g.width() {
            if is_accessible_roll(&g, row, col) {
                out += 1;
            }
        }
    }

    out
}

/// Returns the part2 result
fn part2(text: &str) -> u64 {
    let mut g = Grid::new_char(text).unwrap();
    let mut out = 0;

    loop {
        // note starting point
        let out_start = out;
        let mut new_g = g.clone();
        for row in 0..g.height() {
            for col in 0..g.width() {
                if is_accessible_roll(&g, row, col) {
                    // remove accessible rolls here
                    *new_g.at_mut(row, col).unwrap() = '.';
                    out += 1;
                }
            }
        }
        if out == out_start {
            // done if no more rolls were removed
            break;
        } else {
            // run it back with updated state
            g = new_g;
        }
    }

    out
}

/// Check whether space is an accessible roll.
fn is_accessible_roll(g: &Grid, row: i32, col: i32) -> bool {
    let value = g.at(row, col).unwrap();
    let num_neighbors = count_neighbor_rolls(g, row, col);

    *value == '@' && num_neighbors < 4
}

/// Count neighbors that have rolls `@` in 8 adjacent spots.
fn count_neighbor_rolls(g: &Grid, row: i32, col: i32) -> u64 {
    let mut out = 0;

    for drow in [-1, 0, 1] {
        for dcol in [-1, 0, 1] {
            if drow == 0 && dcol == 0 {
                continue;
            }

            let xrow = row + drow;
            let xcol = col + dcol;
            if let Ok(&'@') = g.at(xrow, xcol) {
                out += 1
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_read_grid() {
        Grid::new_char(TEST1).unwrap();
    }

    #[test]
    fn test_count_neighbors_1() {
        let s = "
.@.
@..
@@@
";
        let g = Grid::new_char(s).unwrap();
        let result = count_neighbor_rolls(&g, 0, 0);
        assert_eq!(result, 2);
        let result = count_neighbor_rolls(&g, 1, 1);
        assert_eq!(result, 5);
        let result = count_neighbor_rolls(&g, 2, 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST1);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST1);
        assert_eq!(result, 43);
    }

    // ...
}

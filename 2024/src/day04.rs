use std::fs;

use anyhow::Result;
use nalgebra::DMatrix;

fn parse(input: &str) -> DMatrix<char> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let nrow = lines.len();
    let ncol = lines[0].len();

    DMatrix::from_fn(nrow, ncol, |row, col| {
        lines[row].chars().collect::<Vec<_>>()[col]
    })
}

fn part1(data: &DMatrix<char>) -> i32 {
    let nrows = data.nrows() as i32;
    let ncols = data.ncols() as i32;
    let orientations = [
        (-1i32, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut num_xmas = 0;

    for start_row in 0..nrows {
        for start_col in 0..ncols {
            'orient: for delta in orientations.iter() {
                let mut row = start_row;
                let mut col = start_col;
                let mut word = data[(row as usize, col as usize)].to_string();
                // let mut coords = vec![(start_row, start_col)];
                for i in 0..3 {
                    row += delta.0;
                    col += delta.1;
                    if row < 0 || nrows <= row || col < 0 || ncols <= col {
                        continue 'orient;
                    }
                    word.push(data[(row as usize, col as usize)]);
                }
                if word == "XMAS" {
                    // println!(
                    //     "start_row = {start_row} / start_col = {start_col} / delta = {delta:?}"
                    // );
                    num_xmas += 1;
                }
            }
        }
    }

    num_xmas
}

fn part2(data: &DMatrix<char>) -> i32 {
    let nrows = data.nrows();
    let ncols = data.ncols();
    let mut count = 0;

    let patterns = ["MS".to_string(), "SM".to_string()];

    for row in 1..nrows - 1 {
        for col in 1..ncols - 1 {
            if data[(row, col)] != 'A' {
                continue;
            }
            let x1 = format!("{}{}", data[(row - 1, col - 1)], data[(row + 1, col + 1)]);
            let x2 = format!("{}{}", data[(row - 1, col + 1)], data[(row + 1, col - 1)]);
            if patterns.contains(&x1) && patterns.contains(&x2) {
                count += 1;
            }
        }
    }

    count
}

// const TEST_INPUT: &str = "MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX";

fn main() -> Result<()> {
    // let input = TEST_INPUT.to_string();
    let input = fs::read_to_string("input-04.txt")?;
    let data = parse(&input);

    // dbg!(&data);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}

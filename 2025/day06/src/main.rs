use anyhow::Result;
use grid::Grid;
use std::{collections::VecDeque, fs};

fn main() -> Result<()> {
    let text = fs::read_to_string("data/06.txt")?;

    println!("{}", part1(&text));
    println!("{}", part2(&text));

    Ok(())
}

/// Returns the part1 result
fn part1(text: &str) -> u64 {
    tasks_from_text(text).iter().map(|task| task.result()).sum()
}

/// Returns the part2 result
fn part2(text: &str) -> u64 {
    tasks_from_text_part2(text)
        .iter()
        .map(|task| task.result())
        .sum()
}

/// Parse the text the oblivious way
///
/// This is for part1, before we learn the orientation of "cephalopod math"
fn tasks_from_text(text: &str) -> Vec<Task> {
    let mut out = Vec::new();

    let mut lines: VecDeque<_> = text.trim().split('\n').collect();
    let ops = lines.pop_back().unwrap().split_whitespace().map(|s| {
        s.trim()
            .maybe_op()
            .unwrap_or_else(|| panic!("unexpected op character {s}"))
    });

    let all_nums: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for (i, op) in ops.enumerate() {
        let nums = all_nums
            .iter()
            .map(|nums| {
                nums[i]
                    .parse()
                    .unwrap_or_else(|_| panic!("could not parse {}", nums[i]))
            })
            .collect();
        out.push(Task { nums, op });
    }

    out
}

/// Parse the text following "cephalopod math" rules
fn tasks_from_text_part2(text: &str) -> Vec<Task> {
    let mut out = Vec::new();

    let grid = Grid::new_char(text).unwrap_or_else(|_| panic!("could not parse:{}", text));

    let op_row = grid.height() - 1;
    let last_col = grid.width() - 1;
    let mut op = grid
        .at(op_row, 0)
        .unwrap()
        .maybe_op()
        .unwrap_or_else(|| panic!("should have an op at ({op_row}, 0)"));
    let mut nums: Vec<u64> = Vec::new();

    for col in 0..=last_col {
        // check the op first in case we're starting a new one
        if let Some(o) = grid.at(op_row, col).unwrap().maybe_op() {
            // this is a new task
            op = o;
        }

        // now collect any digits present in this col
        let digits = (0..op_row)
            .map(|row| *grid.at(row, col).unwrap())
            .collect::<String>();
        let digits = digits.trim();

        // if we got digits, add them to the nums for the current task
        if !digits.is_empty() {
            nums.push(
                digits
                    .parse()
                    .unwrap_or_else(|_| panic!("could not parse '{digits}'")),
            );
        }

        // if we're on an empty col or the last col, finalize the current task
        if digits.is_empty() || col == last_col {
            out.push(Task { nums, op });
            nums = Vec::new();
            continue;
        }
    }

    out
}

/// One addition or multiplication task
#[derive(Clone, Debug)]
struct Task {
    nums: Vec<u64>,
    op: Op,
}

impl Task {
    /// Execute the task
    fn result(&self) -> u64 {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mul => self.nums.iter().product(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

/// Either char or &str will be maybe convertable to Op
trait ToOp {
    /// Get Some(Add), Some(Mul), or None
    fn maybe_op(&self) -> Option<Op>;
}

impl ToOp for char {
    fn maybe_op(&self) -> Option<Op> {
        match self {
            '+' => Some(Op::Add),
            '*' => Some(Op::Mul),
            _ => None,
        }
    }
}

impl ToOp for str {
    fn maybe_op(&self) -> Option<Op> {
        match self {
            "+" => Some(Op::Add),
            "*" => Some(Op::Mul),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST1: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  

";

    #[test]
    fn test_parse_problem() {
        let _p = tasks_from_text(TEST1);
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST1);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST1);
        assert_eq!(result, 3263827);
    }
}

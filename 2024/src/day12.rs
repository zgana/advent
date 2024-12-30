use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use anyhow::Result;

use itertools::Itertools;
use mdraoc::grid::{Delta, Edge, Grid, Loc};

type Block = HashSet<Loc>;

fn get_point_sets_by_letter(grid: &Grid<char>) -> HashMap<char, HashSet<Loc>> {
    let mut out: HashMap<char, HashSet<Loc>> = HashMap::new();

    for (loc, value) in grid.each_item() {
        if !out.contains_key(value) {
            out.insert(*value, HashSet::new());
        }
        out.get_mut(value)
            .expect("char key should be present")
            .insert(loc);
    }

    out
}

fn get_blocks(points: &HashSet<Loc>) -> Vec<Block> {
    let mut points = points.clone();
    let mut blocks = Vec::new();
    let directions = [Delta::DOWN, Delta::RIGHT, Delta::UP, Delta::LEFT];

    // do as many flood fills as we need to
    while !points.is_empty() {
        // arbitrarily start at upper left
        let mut loc = *points
            .iter()
            .min()
            .expect("already checked !points.is_empty()");
        let mut queue = VecDeque::from([loc]);
        let mut block: HashSet<Loc> = HashSet::new();

        while !queue.is_empty() {
            loc = queue
                .pop_front()
                .expect("already checked !queue.is_empty()");
            if !points.contains(&loc) {
                continue;
            }

            block.insert(loc);
            points.remove(&loc);

            for direction in directions {
                let maybe = loc + direction;
                if points.contains(&maybe) {
                    queue.push_back(maybe);
                }
            }
        }

        blocks.push(block);
    }

    blocks
}

fn get_all_blocks(point_sets: HashMap<char, Block>) -> Vec<Block> {
    let mut blocks = Vec::new();

    for point_set in point_sets.values() {
        blocks.extend(get_blocks(point_set));
    }

    blocks
}

fn get_area(block: &Block) -> u32 {
    block.len().try_into().expect("area should fit in u32")
}

fn get_all_edges(block: &Block) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    for loc in block {
        let loc0 = *loc;
        let loc_right = loc0 + Delta::RIGHT;
        let loc_down = loc0 + Delta::DOWN;
        edges.insert(Edge::Vertical(loc0));
        edges.insert(Edge::Vertical(loc_right));
        edges.insert(Edge::Horizontal(loc0));
        edges.insert(Edge::Horizontal(loc_down));
    }

    edges
}

fn get_internal_edges(block: &Block) -> HashSet<Edge> {
    let locs: Vec<&Loc> = block.iter().collect();
    let mut edges = HashSet::new();

    for (i, loc0) in locs.iter().enumerate() {
        for loc1 in locs.iter().skip(i) {
            if let Ok(edge) = Edge::between(**loc0, **loc1) {
                edges.insert(edge);
            }
        }
    }

    // dbg!(block, edges.len());
    edges
}

fn get_boundary(block: &Block) -> HashSet<Edge> {
    get_all_edges(block)
        .difference(&get_internal_edges(block))
        .cloned()
        .collect()
}

fn get_perimeter(block: &Block) -> u32 {
    get_boundary(block).len() as u32
}

fn get_num_sides(block: &Block) -> u32 {
    use Edge::*;

    let boundary = get_boundary(block);
    let len_boundary = boundary.len() as u32;

    let mut edge_sets: HashMap<i32, HashSet<Edge>> = HashMap::new();
    for edge in boundary.iter() {
        match edge {
            Horizontal(loc) => edge_sets.entry(loc.row()).or_default().insert(*edge),
            Vertical(loc) => edge_sets.entry(loc.col()).or_default().insert(*edge),
        };
    }

    let mut num_shared = 0;
    for edge_set in edge_sets.values() {
        let edges: Vec<Edge> = edge_set.iter().sorted().cloned().collect();
        for edge in edges {
            // if the next edge is not part of the set, then no adjacent to count
            if !edge_set.contains(&edge.next()) {
                continue;
            }

            // otherwise, check for perpendicular bisector
            let (test0, test1) = match edge {
                Horizontal(loc) => (
                    Vertical(loc + Delta::RIGHT),
                    Vertical(loc + Delta::RIGHT + Delta::UP),
                ),
                Vertical(loc) => (
                    Horizontal(loc + Delta::DOWN),
                    Horizontal(loc + Delta::DOWN + Delta::LEFT),
                ),
            };
            if boundary.contains(&test0) && boundary.contains(&test1) {
                continue;
            }

            num_shared += 1;
        }
    }

    len_boundary - num_shared
}

fn part1_price(block: &Block) -> u32 {
    get_area(block) * get_perimeter(block)
}

fn part2_price(block: &Block) -> u32 {
    get_area(block) * get_num_sides(block)
}

fn part1(data: &Grid<char>) -> u32 {
    let blocks = get_all_blocks(get_point_sets_by_letter(data));
    blocks.iter().map(part1_price).sum()
}

fn part2(data: &Grid<char>) -> u32 {
    let blocks = get_all_blocks(get_point_sets_by_letter(data));
    blocks.iter().map(part2_price).sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-12.txt")?;
    let data = Grid::parse_char_grid(&input);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST0: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST1: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const TEST4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part1() {
        let grid = Grid::parse_char_grid(TEST0);
        assert_eq!(part1(&grid), 140);
        let grid = Grid::parse_char_grid(TEST1);
        assert_eq!(part1(&grid), 772);
        let grid = Grid::parse_char_grid(TEST2);
        assert_eq!(part1(&grid), 1930);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::parse_char_grid(TEST0);
        assert_eq!(part2(&grid), 80);
        let grid = Grid::parse_char_grid(TEST1);
        assert_eq!(part2(&grid), 436);
        let grid = Grid::parse_char_grid(TEST2);
        assert_eq!(part2(&grid), 1206);
        let grid = Grid::parse_char_grid(TEST3);
        assert_eq!(part2(&grid), 236);
        let grid = Grid::parse_char_grid(TEST4);
        assert_eq!(part2(&grid), 368);
    }
}

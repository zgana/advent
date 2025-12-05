use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

use anyhow::Result;
use mdraoc::grid::{Delta, Grid, Loc};

type Maze = Grid<char>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(&self) -> Delta {
        use Dir::*;
        match self {
            Up => Delta::UP,
            Down => Delta::DOWN,
            Left => Delta::LEFT,
            Right => Delta::RIGHT,
        }
    }

    fn rot_cw(&self) -> Self {
        use Dir::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn rot_ccw(&self) -> Self {
        use Dir::*;
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    loc: Loc,
    dir: Dir,
    cost: u32,
}

impl State {
    fn new(loc: Loc, dir: Dir, cost: u32) -> Self {
        Self { loc, dir, cost }
    }

    fn locdir(&self) -> (Loc, Dir) {
        (self.loc, self.dir)
    }
}

// State orders by cost first, in reverse, to flip BinaryHeap from max-heap to min heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse order by cost
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (self.loc, self.dir).cmp(&(other.loc, other.dir)))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Maze {
    // ...
    Grid::parse_char_grid(input)
}

fn find(maze: &Maze, c: char) -> Option<Loc> {
    for (loc, &val) in maze.each_item() {
        if val == c {
            return Some(loc);
        }
    }

    None
}

fn part1(maze: &Maze) -> u32 {
    use Dir::*;

    let start_loc = find(maze, 'S').expect("should find 'S'");
    let goal_loc = find(maze, 'E').expect("should find 'E'");
    let (loc, dir) = (start_loc, Right);
    let start_state = State { loc, dir, cost: 0 };

    let mut dist: HashMap<(Loc, Dir), u32> = HashMap::new();
    let mut prev: HashMap<(Loc, Dir), Option<State>> = HashMap::new();
    dist.insert((loc, dir), 0);
    prev.insert((loc, dir), None);

    let mut heap = BinaryHeap::new();
    heap.push(start_state);
    while let Some(state) = heap.pop() {
        // don't quit when we reach goal, since another direction of approach might be better
        // ...
        let loc = state.loc;
        let dir = state.dir;
        let cost = state.cost;
        let locdir = (loc, dir);
        if cost > dist[&locdir] {
            continue;
        }

        // try stepping or turning
        let next_states = [
            State::new(loc, dir.rot_cw(), cost + 1000),
            State::new(loc, dir.rot_ccw(), cost + 1000),
            State::new(loc + dir.delta(), dir, cost + 1),
        ];

        for next_state in next_states {
            let next_loc = next_state.loc;
            let next_locdir = next_state.locdir();

            // don't clip through walls
            let c = *maze.at(next_loc);
            if c == '#' {
                continue;
            }

            // don't pursue more expensive routes
            if let Some(existing_cost) = dist.get(&next_locdir) {
                if next_state.cost >= *existing_cost {
                    continue;
                }
            }

            // add next state to queue
            heap.push(next_state.clone());
            dist.insert(next_locdir, next_state.cost);
            prev.insert(next_locdir, Some(state.clone()));
        }
    }

    *dist
        .iter()
        .filter(|((l, _d), _v)| *l == goal_loc)
        .map(|((_l, _d), v)| v)
        .min()
        .expect("should find a minimum distance")
}

fn trace_all_paths(prev: &HashMap<State, HashSet<State>>, end: Vec<State>) -> Vec<Loc> {
    let mut out: HashSet<Loc> = HashSet::new();

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.extend(end.iter().cloned());
    while let Some(state) = queue.pop_front() {
        // println!("loc = {loc}");
        out.insert(state.loc);
        if !prev.contains_key(&state) {
            continue;
        }
        for s in prev.get(&state).expect("key should be present") {
            queue.push_back(s.clone());
        }
    }
    out.iter().cloned().collect()
}

fn part2(maze: &Maze) -> u32 {
    use Dir::*;

    let start_loc = find(maze, 'S').expect("should find 'S'");
    let goal_loc = find(maze, 'E').expect("should find 'E'");
    let (loc, dir) = (start_loc, Right);
    let start_state = State { loc, dir, cost: 0 };

    let mut dist: HashMap<(Loc, Dir), u32> = HashMap::new();
    let mut prev: HashMap<State, HashSet<State>> = HashMap::new();
    dist.insert((loc, dir), 0);
    prev.insert(start_state.clone(), HashSet::new());

    let mut heap = BinaryHeap::new();
    heap.push(start_state);
    while let Some(state) = heap.pop() {
        let loc = state.loc;
        let dir = state.dir;
        let cost = state.cost;
        let locdir = (loc, dir);

        // if this is the goal, move on
        if loc == goal_loc {
            continue;
        }

        // if we already found a better path, move on
        if cost > dist[&locdir] {
            continue;
        }

        // try stepping or turning
        let next_states = [
            State::new(loc, dir.rot_cw(), cost + 1000),
            State::new(loc, dir.rot_ccw(), cost + 1000),
            State::new(loc + dir.delta(), dir, cost + 1),
        ];

        for next_state in next_states {
            let next_loc = next_state.loc;
            let next_locdir = next_state.locdir();

            // don't clip through walls
            let c = *maze.at(next_loc);
            if c == '#' {
                continue;
            }

            // don't pursue more expensive routes
            // but DO pursue equally expensive route
            if let Some(existing_cost) = dist.get(&next_locdir) {
                if next_state.cost > *existing_cost {
                    continue;
                }
            }

            // add next state to queue
            heap.push(next_state.clone());
            dist.insert(next_locdir, next_state.cost);

            // if we're taking a step, take note of the path taken
            prev.entry(next_state).or_default().insert(state.clone());
        }
    }

    // gather ending states
    let all_endings = dist
        .iter()
        .filter(|((l, _d), _c)| *l == goal_loc)
        .collect::<Vec<_>>();

    // find best ending states
    let min_cost = all_endings
        .iter()
        .map(|((_l, _d), c)| c)
        .min()
        .expect("should have a min cost");
    let ending_states = all_endings
        .iter()
        .filter(|((_l, _d), c)| c == min_cost)
        .map(|((l, d), c)| State::new(*l, *d, **c))
        .collect::<Vec<_>>();

    // find visited locs
    let visited = trace_all_paths(&prev, ending_states);

    // // show visited locs
    // let viz = maze.transform(|(loc, &val)| {
    //     if visited.contains(&loc) {
    //         'O'
    //     } else if val == '#' {
    //         '#'
    //     } else {
    //         '.'
    //     }
    // });
    // println!("\n\n{viz}\n");

    visited.len().try_into().expect("result should fit in u32")
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input-16.txt")?;
    let maze = parse(&input);

    dbg!(maze
        .each_item()
        .filter(|(_l, &v)| v == '.')
        .collect::<Vec<_>>()
        .len());

    println!("{}", part1(&maze));
    println!("{}", part2(&maze));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_find() {
        let maze = parse(TEST);
        let expected_s = Loc(13, 1);
        let expected_e = Loc(1, 13);
        assert_eq!(find(&maze, 'S').unwrap(), expected_s);
        assert_eq!(find(&maze, 'E').unwrap(), expected_e);
    }

    #[test]
    fn test_find2() {
        let maze = parse(TEST2);
        let expected_s = Loc(15, 1);
        let expected_e = Loc(1, 15);
        assert_eq!(find(&maze, 'S').unwrap(), expected_s);
        assert_eq!(find(&maze, 'E').unwrap(), expected_e);
    }

    #[test]
    fn test_part1_small() {
        let maze = parse(TEST);
        let result = part1(&maze);
        let expected = 7036;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1_large() {
        let maze = parse(TEST2);
        let result = part1(&maze);
        let expected = 11048;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_small() {
        let maze = parse(TEST);
        let result = part2(&maze);
        let expected = 45;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_large() {
        let maze = parse(TEST2);
        let result = part2(&maze);
        let expected = 64;
        assert_eq!(result, expected);
    }
}

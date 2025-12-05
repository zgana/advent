pub mod grid {
    use std::{
        fmt,
        ops::{Add, Sub},
    };

    use anyhow::Result;
    use itertools::Itertools;

    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Loc(pub i32, pub i32);

    impl Loc {
        pub fn new(row: i32, col: i32) -> Self {
            Self(row, col)
        }

        pub fn from_usize(row: usize, col: usize) -> Self {
            let row = row.try_into().expect("row must fit in i32");
            let col = col.try_into().expect("col must fit in i32");
            Self(row, col)
        }

        pub fn from_u32(row: u32, col: u32) -> Self {
            let row = row.try_into().expect("row must fit in i32");
            let col = col.try_into().expect("col must fit in i32");
            Self(row, col)
        }

        pub fn from_u64(row: u64, col: u64) -> Self {
            let row = row.try_into().expect("row must fit in i32");
            let col = col.try_into().expect("col must fit in i32");
            Self(row, col)
        }

        pub fn from_i64(row: i64, col: i64) -> Self {
            let row = row.try_into().expect("row must fit in i32");
            let col = col.try_into().expect("col must fit in i32");
            Self(row, col)
        }

        pub fn row(&self) -> i32 {
            self.0
        }

        pub fn row_u32(&self) -> u32 {
            self.0 as u32
        }

        pub fn row_usize(&self) -> usize {
            self.0 as usize
        }

        pub fn col(&self) -> i32 {
            self.1
        }

        pub fn col_u32(&self) -> u32 {
            self.1 as u32
        }

        pub fn col_usize(&self) -> usize {
            self.1 as usize
        }
    }

    impl fmt::Display for Loc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Loc({}, {})", self.row(), self.col())
        }
    }

    impl fmt::Debug for Loc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Loc({}, {})", self.row(), self.col())
        }
    }

    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Delta(pub i32, pub i32);

    impl Delta {
        pub fn new(nrows: i32, ncols: i32) -> Self {
            Self(nrows, ncols)
        }

        pub fn nrows(&self) -> i32 {
            self.0
        }

        pub fn ncols(&self) -> i32 {
            self.1
        }

        pub fn rot_cw(&self) -> Self {
            Self(self.ncols(), -self.nrows())
        }

        pub fn rot_ccw(&self) -> Self {
            Self(-self.ncols(), self.nrows())
        }

        pub const UP: Self = Self(-1, 0);
        pub const DOWN: Self = Self(1, 0);
        pub const LEFT: Self = Self(0, -1);
        pub const RIGHT: Self = Self(0, 1);
    }

    impl fmt::Display for Delta {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Delta({}, {})", self.nrows(), self.ncols())
        }
    }

    impl Sub for Loc {
        type Output = Delta;

        fn sub(self, rhs: Loc) -> Self::Output {
            let nrows = self.row() - rhs.row();
            let ncols = self.col() - rhs.col();

            Delta::new(nrows, ncols)
        }
    }

    impl Add<Delta> for Loc {
        type Output = Loc;

        fn add(self, rhs: Delta) -> Self::Output {
            let row = self.row() + rhs.nrows();
            let col = self.col() + rhs.ncols();

            Loc::new(row, col)
        }
    }

    #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Edge {
        Horizontal(Loc),
        Vertical(Loc),
    }

    impl Edge {
        pub fn beside(a: Loc, delta: Delta) -> Result<Self, NoEdgeError> {
            let b = a + delta;
            match delta {
                Delta::RIGHT => Ok(Edge::Vertical(b)),
                Delta::DOWN => Ok(Edge::Horizontal(b)),
                Delta::LEFT => Ok(Edge::Vertical(a)),
                Delta::UP => Ok(Edge::Horizontal(a)),
                _ => Err(NoEdgeError(a, b)),
            }
        }

        pub fn between(a: Loc, b: Loc) -> Result<Self, NoEdgeError> {
            let delta = b - a;
            match delta {
                Delta::RIGHT => Ok(Edge::Vertical(b)),
                Delta::DOWN => Ok(Edge::Horizontal(b)),
                Delta::LEFT => Ok(Edge::Vertical(a)),
                Delta::UP => Ok(Edge::Horizontal(a)),
                _ => Err(NoEdgeError(a, b)),
            }
        }

        pub fn next(&self) -> Self {
            match self {
                Edge::Horizontal(loc) => Edge::Horizontal(*loc + Delta::RIGHT),
                Edge::Vertical(loc) => Edge::Vertical(*loc + Delta::DOWN),
            }
        }
    }

    #[derive(Debug)]
    pub struct NoEdgeError(Loc, Loc);

    impl fmt::Display for NoEdgeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "No edge between {} and {}", self.0, self.1)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Grid<T> {
        nrows: u32,
        ncols: u32,
        data: Vec<T>,
    }

    impl Grid<char> {
        pub fn parse_char_grid(input: &str) -> Self {
            let lines: Vec<&str> = input.trim().split('\n').collect();
            let nrows = lines.len();
            let ncols = lines[0].len();
            let mut data = Grid::new_usize(nrows, ncols);

            for (row, line) in lines.iter().enumerate() {
                for (col, char) in line.chars().enumerate() {
                    *data.at_usize_mut(row, col) = char;
                }
            }

            data
        }
    }

    // impl fmt::Debug for Grid<char> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         for row in 0..self.nrows {
    //             for col in 0..self.ncols {
    //                 write!(f, "{}", self.at_u32(row, col))?;
    //             }
    //             writeln!(f)?;
    //         }
    //         Ok(())
    //     }
    // }

    impl fmt::Display for Grid<char> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in 0..self.nrows {
                for col in 0..self.ncols {
                    write!(f, "{}", self.at_u32(row, col))?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl<T: Default + Clone> Grid<T> {
        pub fn new(nrows: u32, ncols: u32) -> Self {
            Self::from_value(nrows, ncols, T::default())
        }

        pub fn new_u64(nrows: u64, ncols: u64) -> Self {
            Self::from_value_u64(nrows, ncols, T::default())
        }

        pub fn new_usize(nrows: usize, ncols: usize) -> Self {
            Self::from_value_usize(nrows, ncols, T::default())
        }

        pub fn from_array(nrows: u32, ncols: u32, input: &[&[T]]) -> Self {
            let mut out = Grid::new(nrows, ncols);
            for loc in out.each_loc() {
                let row = loc.row_usize();
                let col = loc.col_usize();
                *out.at_mut(loc) = input[row][col].clone();
            }

            out
        }

        pub fn from_value(nrows: u32, ncols: u32, value: T) -> Self {
            let size: usize = (nrows * ncols)
                .try_into()
                .expect("nrows * ncols must fit in a usize");
            let data = vec![value; size];
            Self { nrows, ncols, data }
        }

        pub fn from_value_u64(nrows: u64, ncols: u64, value: T) -> Self {
            let nrows: u32 = nrows.try_into().expect("nrows must fit in an u32");
            let ncols: u32 = ncols.try_into().expect("ncols must fit in an u32");
            let size: usize = (nrows * ncols)
                .try_into()
                .expect("nrows * ncols must fit in a usize");
            let data = vec![value; size];
            Self { nrows, ncols, data }
        }

        pub fn from_value_usize(nrows: usize, ncols: usize, value: T) -> Self {
            let nrows: u32 = nrows.try_into().expect("nrows must fit in u32");
            let ncols: u32 = ncols.try_into().expect("ncols must fit in u32");
            let size: usize = (nrows * ncols)
                .try_into()
                .expect("nrows * ncols must fit in a usize");
            let data = vec![value; size];
            Self { nrows, ncols, data }
        }

        pub fn each_loc(&self) -> impl Iterator<Item = Loc> {
            (0..self.nrows)
                .cartesian_product(0..self.ncols)
                .map(|(r, c)| Loc::from_u32(r, c))
        }

        pub fn each_value(&self) -> impl Iterator<Item = &T> {
            self.data.iter()
        }

        pub fn each_value_mut(&mut self) -> impl Iterator<Item = &mut T> {
            self.data.iter_mut()
        }

        pub fn each_item(&self) -> impl Iterator<Item = (Loc, &T)> {
            self.each_loc().zip(self.each_value())
        }

        pub fn each_item_mut(&mut self) -> impl Iterator<Item = (Loc, &mut T)> {
            self.each_loc().zip(self.each_value_mut())
        }

        pub fn transform<O, F>(&self, f: F) -> Grid<O>
        where
            F: Fn((Loc, &T)) -> O,
            O: Clone + Default,
        {
            let mut out = Grid::new(self.nrows, self.ncols);
            for (loc, val) in self.each_item() {
                *out.at_mut(loc) = f((loc, val)).clone();
            }
            out
        }

        pub fn at(&self, loc: Loc) -> &T {
            self.at_u32(loc.row_u32(), loc.col_u32())
        }

        pub fn at_u32(&self, row: u32, col: u32) -> &T {
            let index = self.u32_to_index(row, col);
            &self.data[index]
        }

        pub fn at_u64(&self, row: u64, col: u64) -> &T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32(row, col)
        }

        pub fn at_usize(&self, row: usize, col: usize) -> &T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32(row, col)
        }

        pub fn at_i32(&self, row: i32, col: i32) -> &T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32(row, col)
        }

        pub fn at_i64(&self, row: i64, col: i64) -> &T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32(row, col)
        }

        pub fn at_mut(&mut self, loc: Loc) -> &mut T {
            self.at_u32_mut(loc.row_u32(), loc.col_u32())
        }

        pub fn at_u32_mut(&mut self, row: u32, col: u32) -> &mut T {
            let index = self.u32_to_index(row, col);
            &mut self.data[index]
        }

        pub fn at_u64_mut(&mut self, row: u64, col: u64) -> &mut T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32_mut(row, col)
        }

        pub fn at_usize_mut(&mut self, row: usize, col: usize) -> &mut T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32_mut(row, col)
        }

        pub fn at_i32_mut(&mut self, row: i32, col: i32) -> &mut T {
            let row = row.try_into().expect("row must fit in an i32");
            let col = col.try_into().expect("col must fit in an i32");
            self.at_u32_mut(row, col)
        }

        pub fn at_i64_mut(&mut self, row: i64, col: i64) -> &mut T {
            let row = row.try_into().expect("row must fit in an i64");
            let col = col.try_into().expect("col must fit in an i64");
            self.at_u32_mut(row, col)
        }

        pub fn u32_to_index(&self, row: u32, col: u32) -> usize {
            let nrows = self.nrows;
            let ncols = self.ncols;
            if row >= nrows {
                panic!("row ({row}) should be less than nrows ({nrows})");
            }
            if col >= ncols {
                panic!("col ({col}) should be less than ncols ({ncols})");
            }
            (row * self.ncols + col)
                .try_into()
                .expect("index must fit in a usize")
        }

        pub fn nrows(&self) -> u32 {
            self.nrows
        }

        pub fn ncols(&self) -> u32 {
            self.ncols
        }

        pub fn raw_data(&self) -> &Vec<T> {
            &self.data
        }

        pub fn size(&self) -> usize {
            (self.nrows * self.ncols)
                .try_into()
                .expect("nrows * ncols must fit in a usize")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grid::*;

    #[test]
    fn test_loc_new() {
        Loc::new(37, 73);
    }

    #[test]
    fn test_loc_01() {
        let loc = Loc(37, 73);
        assert_eq!(loc.row(), 37);
        assert_eq!(loc.col(), 73);
    }

    #[test]
    fn test_loc_01_i32() {
        let loc = Loc(37, 73);
        assert_eq!(loc.row(), 37);
        assert_eq!(loc.col(), 73);
    }

    #[test]
    fn test_loc_sub() {
        let loc1 = Loc::new(1, 2);
        let loc2 = Loc::new(3, 6);
        let expected = Delta::new(2, 4);
        assert_eq!(loc2 - loc1, expected);
    }

    #[test]
    fn test_delta_new() {
        Delta::new(-37, 37);
    }

    #[test]
    fn test_delta_01() {
        let d = Delta(-37, 37);
        assert_eq!(d.nrows(), -37);
        assert_eq!(d.ncols(), 37);
    }

    #[test]
    fn test_delta_rot_cw() {
        let (left, right, up, down) = (Delta::LEFT, Delta::RIGHT, Delta::UP, Delta::DOWN);
        assert_eq!(left.rot_cw(), up);
        assert_eq!(up.rot_cw(), right);
        assert_eq!(right.rot_cw(), down);
        assert_eq!(down.rot_cw(), left);
    }

    #[test]
    fn test_delta_rot_ccw() {
        let (left, right, up, down) = (Delta::LEFT, Delta::RIGHT, Delta::UP, Delta::DOWN);
        assert_eq!(left.rot_ccw(), down);
        assert_eq!(down.rot_ccw(), right);
        assert_eq!(right.rot_ccw(), up);
        assert_eq!(up.rot_ccw(), left);
    }

    #[test]
    fn test_loc_delta_add() {
        let loc = Loc::new(1, 2);
        let delta = Delta::new(3, 6);
        let expected = Loc::new(4, 8);
        assert_eq!(loc + delta, expected);
    }

    #[test]
    fn test_edge_beside_horizontal() {
        let a = Loc::new(2, 3);
        let delta = Delta::DOWN;
        let b = a + delta;
        let edge = Edge::beside(a, delta).unwrap();
        let expected = Edge::Horizontal(b);
        assert_eq!(edge, expected);

        let delta = Delta::UP;
        let edge = Edge::beside(a, delta).unwrap();
        let expected = Edge::Horizontal(a);
        assert_eq!(edge, expected);
    }

    #[test]
    fn test_edge_beside_vertical() {
        let a = Loc::new(2, 3);
        let delta = Delta::RIGHT;
        let b = a + delta;
        let edge = Edge::beside(a, delta).unwrap();
        let expected = Edge::Vertical(b);
        assert_eq!(edge, expected);

        let delta = Delta::LEFT;
        let edge = Edge::beside(a, delta).unwrap();
        let expected = Edge::Vertical(a);
        assert_eq!(edge, expected);
    }

    #[test]
    fn test_edge_between_horizontal() {
        let a = Loc::new(2, 3);
        let b = a + Delta::DOWN;

        let edge = Edge::between(a, b).unwrap();
        let expected = Edge::Horizontal(b);
        assert_eq!(edge, expected);

        let edge = Edge::between(b, a).unwrap();
        let expected = Edge::Horizontal(b);
        assert_eq!(edge, expected);
    }

    #[test]
    fn test_edge_between_vertical() {
        let a = Loc::new(2, 3);
        let b = a + Delta::RIGHT;

        let edge = Edge::between(a, b).unwrap();
        let expected = Edge::Vertical(b);
        assert_eq!(edge, expected);

        let edge = Edge::between(b, a).unwrap();
        let expected = Edge::Vertical(b);
        assert_eq!(edge, expected);
    }

    #[test]
    fn test_grid_new_i32() {
        let g: Grid<i32> = Grid::new(3, 3);
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], 0);
        }
    }

    #[test]
    fn test_grid_new_u32() {
        let g: Grid<u32> = Grid::new(3, 3);
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], 0);
        }
    }

    #[test]
    fn test_grid_new_char() {
        let g: Grid<char> = Grid::new(3, 3);
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], '\0');
        }
    }

    #[test]
    fn test_grid_from_value_i32() {
        let g: Grid<i32> = Grid::from_value(3, 3, -37);
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], -37);
        }
    }

    #[test]
    fn test_grid_from_value_u32() {
        let g: Grid<u32> = Grid::from_value(3, 3, 37);
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], 37);
        }
    }

    #[test]
    fn test_grid_from_value_char() {
        let g: Grid<char> = Grid::from_value(3, 3, 'α');
        for i in 0..9 {
            assert_eq!(g.raw_data()[i], 'α');
        }
    }

    #[test]
    fn test_grid_from_array_u8() {
        let input = [&[0u8, 1, 2] as &[u8], &[3, 4, 5], &[6, 7, 8]];
        let g = Grid::from_array(3, 3, &input[..]);
        for (i, (_loc, &val)) in g.each_item().enumerate() {
            assert_eq!(val, i.try_into().expect("should fit in u8"));
        }
    }

    #[test]
    fn test_grid_size() {
        let g: Grid<u8> = Grid::new(3, 7);
        assert_eq!(g.size(), 21);
    }

    #[test]
    fn test_grid_u32_to_index() {
        let g: Grid<u8> = Grid::new(3, 7);
        let index = g.u32_to_index(2, 6);
        assert_eq!(index, 2 * 7 + 6);
    }

    #[test]
    #[should_panic]
    fn test_grid_u32_to_index_panic() {
        let g: Grid<u8> = Grid::new(3, 7);
        g.u32_to_index(3, 7);
    }

    #[test]
    fn test_grid_transform_char_char() {
        let s = "
abc
def
ghi";
        let g = Grid::parse_char_grid(s.trim());
        let s2 = "
ABC
DEF
GHI";
        let expected = Grid::parse_char_grid(s2);
        let g2 = g.transform(|(_loc, c)| c.to_ascii_uppercase());
        assert_eq!(g2, expected);
    }

    #[test]
    fn test_grid_transform_int_int() {
        let g: Grid<i32> = Grid::new(3, 3);
        let g2 = g.transform(|(loc, val)| loc.row() + loc.col());
        let expected = &[&[0, 1, 2] as &[i32], &[1, 2, 3], &[2, 3, 4]];
        assert_eq!(g2, Grid::from_array(3, 3, expected));
    }

    #[test]
    fn test_grid_at() {
        let mut g: Grid<u8> = Grid::new(3, 7);
        let loc = Loc(2, 6);
        *g.at_mut(loc) = 37;
        assert_eq!(*g.at(loc), 37);
    }

    #[test]
    fn test_grid_at_u32() {
        let mut g: Grid<u8> = Grid::new(3, 7);
        *g.at_u32_mut(2, 6) = 37;
        assert_eq!(*g.at_u32(2, 6), 37);
    }

    #[test]
    fn test_grid_at_u64() {
        let mut g: Grid<u8> = Grid::new(3, 7);
        *g.at_u64_mut(2, 6) = 37;
        assert_eq!(*g.at_u32(2, 6), 37);
    }

    #[test]
    fn test_grid_at_usize() {
        let mut g: Grid<u8> = Grid::new(3, 7);
        *g.at_u64_mut(2, 6) = 37;
        assert_eq!(*g.at_u32(2, 6), 37);
    }

    #[test]
    fn test_grid_at_i32() {
        let mut g: Grid<u8> = Grid::new(3, 7);
        *g.at_i32_mut(2, 6) = 37;
        assert_eq!(*g.at_i32(2, 6), 37);
    }

    #[test]
    #[should_panic]
    fn test_grid_at_panic() {
        let g: Grid<u8> = Grid::new(3, 7);
        let loc = Loc(3, 7);
        g.at(loc);
    }

    #[test]
    #[should_panic]
    fn test_grid_at_i32_panic() {
        let g: Grid<u8> = Grid::new(3, 7);
        g.at_i32(-1, 0);
    }

    //
}

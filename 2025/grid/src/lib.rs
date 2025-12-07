use std::fmt;

/// A 2D grid of char, u8, or whatever.
#[derive(Clone, Debug)]
pub struct Grid<T>
where
    T: Copy,
{
    width: usize,
    height: usize,
    elems: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct BoundsError(pub String);

#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl<T> Grid<T>
where
    T: Copy + fmt::Display,
{
    /// Creates a `Grid` from a text representation.
    ///
    /// TODO:
    ///     - Do we want to handle n-wide grid entries from text input?
    ///     - What about delimited input, e.g. comma or tab separated values?
    pub fn from_text(text: &str, f: impl Fn(char) -> T) -> Result<Self, ParseError> {
        let lines: Vec<_> = text.trim_matches('\n').split('\n').collect();

        let height = lines.len();
        if height == 0 {
            return Err(ParseError("found zero lines in input".to_string()));
        }

        let width = lines.first().unwrap().len();
        if width == 0 {
            return Err(ParseError("found zero columns in input".to_string()));
        }
        for line in lines.iter().skip(1) {
            if line.len() != width {
                return Err(ParseError(
                    "found mismatched line lengths in input".to_string(),
                ));
            }
        }

        let mut elems = Vec::with_capacity(width * height);
        for line in lines.iter() {
            for c in line.chars() {
                elems.push(f(c));
            }
        }

        Ok(Self {
            width,
            height,
            elems,
        })
    }

    /// Creates a String representation of the grid with horizontal and vertical padding.
    pub fn as_padded_string(&self, w: usize, h: usize) -> String {
        let w = w + 1;
        let h = h + 1;
        let mut out = String::with_capacity(self.capacity());
        for row in 0..self.height as i32 {
            for col in 0..self.width as i32 {
                out += &format!("{:w$}", self.at(row, col).unwrap());
            }
            for _ in 0..h {
                out += "\n";
            }
        }
        out
    }

    /// String representation of the grid using one character per entry.
    pub fn as_dense_string(&self) -> String {
        self.as_padded_string(0, 0)
    }

    /// Ref to the value at some (row, col).
    pub fn at(&self, row: i32, col: i32) -> Result<&T, BoundsError> {
        let index = self.position_to_index(row, col)?;
        Ok(&self.elems[index])
    }

    /// Mutable ref to the value at some (row, col).
    pub fn at_mut(&mut self, row: i32, col: i32) -> Result<&mut T, BoundsError> {
        let index = self.position_to_index(row, col)?;
        Ok(&mut self.elems[index])
    }

    /// Whether (row, col) are in bounds.
    pub fn is_inbounds(&self, row: i32, col: i32) -> bool {
        0 <= row && (row as usize) < self.height && 0 <= col && (col as usize) < self.width
    }

    /// 1D index corresponding to a position (row, col).
    pub fn position_to_index(&self, row: i32, col: i32) -> Result<usize, BoundsError> {
        if !self.is_inbounds(row, col) {
            return Err(BoundsError(format!(
                "({row}, {col}) is not in bounds for {}x{} grid",
                self.height, self.width
            )));
        }

        let row = row as usize;
        let col = col as usize;
        Ok(row * self.width + col)
    }

    /// Capacity of the grid (width x height).
    pub fn capacity(&self) -> usize {
        self.width * self.height
    }

    /// Height of the grid.
    pub fn height(&self) -> i32 {
        self.height as i32
    }

    /// Width of the grid.
    pub fn width(&self) -> i32 {
        self.width as i32
    }

    /// Elements of the grid.
    pub fn elems(&self) -> &Vec<T> {
        &self.elems
    }
}

impl Grid<char> {
    /// Creates a `Grid<char>` from a text representation.
    pub fn new_char(text: &str) -> Result<Self, ParseError> {
        Self::from_text(text, |c| c)
    }
}

impl Grid<u8> {
    /// Creates a `Grid<u8>` from a text representation.
    pub fn new_u8(text: &str) -> Result<Self, ParseError> {
        Self::from_text(text, |c| c.to_string().parse().unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_CHAR_1: &str = "
abcabcabc
defdefdef
ghighighi
";

    const TEST_NUM_1: &str = "
123123123
456456456
789789789
";

    #[test]
    fn test_grid_new_char() {
        Grid::new_char(TEST_CHAR_1).unwrap();
    }

    #[test]
    fn test_grid_new_u8() {
        Grid::new_u8(TEST_NUM_1).unwrap();
    }

    #[test]
    fn test_grid_dense_string() {
        let g: Grid<u8> = Grid::new_u8(TEST_NUM_1).unwrap();
        println!("\n{}", g.as_dense_string());
    }

    #[test]
    fn test_grid_padded_string() {
        let g: Grid<u8> = Grid::new_u8(TEST_NUM_1).unwrap();
        println!("\n{}", g.as_padded_string(2, 1));
    }

    // ...
}

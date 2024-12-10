//! An implementation of a 2-dimensional grid.
//! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`]:
//!
//! ```
//!   # use advent_of_code::utils::*;
//!
//!   let mut grid = Grid::parse("12\n34");
//!   let point = Point::new(0, 0);
//!
//!   let foo = grid[point];
//!   assert_eq!(foo, b'1');
//!
//!   grid[point] = foo + 1;
//!   assert_eq!(grid[point], b'2');
//! ```

use crate::utils::point::*;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};

// GRID
#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub field: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: u16, height: u16, val: T) -> Self {
        Grid {
            width: width as i32,
            height: height as i32,
            field: vec![val; (width * height) as usize],
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, needle: T) -> Option<Point> {
        let pos = self
            .field
            .iter()
            .position(|v| *v == needle)
            .map(|pos| self.get_coord_from_arr_index(pos));

        pos
    }
}

impl<T> Grid<T> {
    pub fn new_same_size_with<U: Copy>(&self, new_val: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            field: vec![new_val; (self.width * self.height) as usize],
        }
    }

    pub fn get_coord_from_arr_index(&self, index: usize) -> Point {
        Point::new(index as i32 % self.width, index as i32 / self.height)
    }

    #[inline]
    pub fn contains(&self, coord: Point) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw_field = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let width = raw_field[0].len() as i32;
        let height = raw_field.len() as i32;
        let mut field = Vec::with_capacity((width * height) as usize);

        for bytes in raw_field.into_iter() {
            field.extend_from_slice(bytes);
        }

        Self {
            width,
            height,
            field,
        }
    }

    /// Parses the input string into a [`Grid`]
    /// while collecting each unique byte's position in a `HashMap<unique_val, Vec<found_positions>>`.
    /// Ignores bytes in `ignored` array. If you want to record all bytes pass in an empty array:
    /// ```
    /// # use advent_of_code::utils::*;
    /// # use std::collections::HashMap;
    ///
    /// let input = "012";
    /// // Doesn't record unique positions for '2'.
    /// let (_grid, positions) = Grid::parse_with_unique_positions(input, b"2");
    /// let expected_positions = HashMap::from([(b'0', vec![Point::new(0,0)]), (b'1', vec![Point::new(1,0)])]);
    /// assert_eq!(expected_positions, positions);
    ///
    /// // Records unique positions for all bytes.
    /// let (_grid, positions) = Grid::parse_with_unique_positions(input, b"");
    /// let expected_positions = HashMap::from([
    ///     (b'0', vec![Point::new(0,0)]),
    ///     (b'1', vec![Point::new(1,0)]),
    ///     (b'2', vec![Point::new(2, 0)])
    /// ]);
    /// assert_eq!(expected_positions, positions);
    ///
    /// let input = "0001";
    /// let (_grid, positions) = Grid::parse_with_unique_positions(input, b"1");
    /// let expected_positions =
    ///             HashMap::from([
    ///                 (b'0', vec![Point::new(0,0), Point::new(1,0), Point::new(2,0)])
    ///             ]);
    /// assert_eq!(expected_positions, positions);
    /// ```
    pub fn parse_with_unique_positions(
        input: &str,
        ignored: &[u8],
    ) -> (Self, HashMap<u8, Vec<Point>>) {
        let raw_field = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let width = raw_field[0].len() as i32;
        let height = raw_field.len() as i32;
        let mut field = Vec::with_capacity((width * height) as usize);

        let mut uniq_pos = HashMap::new();
        for (y, bytes) in raw_field.into_iter().enumerate() {
            for (x, byte) in bytes.iter().enumerate() {
                if !ignored.contains(byte) {
                    let position: Point = Point::new(x as i32, y as i32);
                    uniq_pos
                        .entry(*byte)
                        .and_modify(|pos_arr: &mut Vec<_>| pos_arr.push(position))
                        .or_insert(vec![position]);
                }
            }

            field.extend_from_slice(bytes);
        }

        (
            Self {
                width,
                height,
                field,
            },
            uniq_pos,
        )
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.field[(index.y * self.width + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.field[(index.y * self.width + index.x) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_working() {
        let grid = Grid::new(4, 3, b'.');
        let test_field = vec![b'.'; 12];

        assert_eq!(test_field, grid.field);

        let valid_coords = Point::new(3, 2);
        assert!(grid.contains(valid_coords));
        let invalid_coords = Point::new(4, 2);
        assert!(!grid.contains(invalid_coords));

        let grid = grid.new_same_size_with(b'#');
        let test_field = vec![b'#'; 12];
        assert_eq!(test_field, grid.field);
    }

    #[test]
    fn grid_parse_working() {
        let input = "012\n345\n678";
        let mut grid = Grid::parse(input);
        let mid_coord = Point::new(1, 1);
        assert_eq!(mid_coord, grid.find(b'4').unwrap());
        assert_eq!(grid[mid_coord], b'4');
        grid[mid_coord] = b'^';
        assert_eq!(mid_coord, grid.find(b'^').unwrap());
        assert_eq!(grid[mid_coord], b'^');
    }

    #[test]
    fn grid_parse_with_positions_working() {
        use crate::utils::*;
        use std::collections::HashMap;

        let input = "012";
        // Doesn't record a unique position for '2'.
        let (_grid, positions) = Grid::parse_with_unique_positions(input, b"2");
        let expected_positions = HashMap::from([
            (b'0', vec![Point::new(0, 0)]),
            (b'1', vec![Point::new(1, 0)]),
        ]);
        assert_eq!(expected_positions, positions);

        // Records a unique positions for all chars.
        let (_grid, positions) = Grid::parse_with_unique_positions(input, b"");
        let expected_positions = HashMap::from([
            (b'0', vec![Point::new(0, 0)]),
            (b'1', vec![Point::new(1, 0)]),
            (b'2', vec![Point::new(2, 0)]),
        ]);
        assert_eq!(expected_positions, positions);

        let input = "0001";
        let (_grid, positions) = Grid::parse_with_unique_positions(input, b"1");
        let expected_positions = HashMap::from([(
            b'0',
            vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)],
        )]);
        assert_eq!(expected_positions, positions);
    }
}

use std::{fmt, iter::Zip, mem, ops::{Index, IndexMut}};

/// Dynamically allocated 2d array
#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    cols: usize,
    rows: usize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    /// Allocates a new grid with default values for each cell
    pub fn new(cols: usize, rows: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            cols,
            rows,
            data: vec![Default::default(); cols * rows].into(),
        }
    }

    pub fn from_slice(cols: usize, rows: usize, slice: &[T]) -> Self
    where
        T: Default + Clone,
    {
        assert_eq!(cols * rows, slice.len());
        let mut g = Self::new(cols, rows);
        g.data.clone_from_slice(slice);
        g
    }

    pub fn clone_from(&mut self, src: &Self)
    where
        T: Clone,
    {
        assert_eq!(self.cols, src.cols);
        assert_eq!(self.rows, src.rows);
        self.data.clone_from_slice(&src.data);
    }

    /// Sets each cell to the default value
    pub fn clear(&mut self)
    where
        T: Default,
    {
        for (_, c) in self {
            *c = Default::default();
        }
    }

    /// Swaps values with another grid, given its dimensions are the same
    pub fn swap(&mut self, other: &mut Self) {
        assert!(self.cols == other.cols && self.rows == other.rows);
        mem::swap(&mut self.data, &mut other.data);
    }

    /// Returns an iterator over all index pairs `(col, row)` corresponding to
    /// cells of the grid
    pub const fn indices(&self) -> Indices {
        Indices {
            cols: self.cols,
            rows: self.rows,
            col: 0,
            row: 0,
        }
    }



    pub const fn cols(&self) -> usize { self.cols }
    pub const fn rows(&self) -> usize { self.rows }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        for row in 0..self.rows {
            if row > 0 {
                f.write_str(",\n ")?;
            }

            f.debug_list()
                .entries((0..self.cols).map(|col| &self.data[row * self.cols + col]))
                .finish()?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (col, row) = index;
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (col, row) = index;
        &mut self.data[row * self.cols + col]
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type IntoIter = Zip<Indices, std::slice::Iter<'a, T>>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.indices().zip(self.data.iter())
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type IntoIter = Zip<Indices, std::slice::IterMut<'a, T>>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.indices().zip(self.data.iter_mut())
    }
}

pub struct Indices {
    col: usize,
    row: usize,
    cols: usize,
    rows: usize,
}

impl Iterator for Indices {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.rows {
            return None;
        } else {
            let item = (self.col, self.row);

            self.col += 1;
            if self.col >= self.cols {
                self.col = 0;
                self.row += 1;
            }

            Some(item)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let grid = Grid::<()>::new(0, 0);
        assert_eq!(grid.cols(), 0);
        assert_eq!(grid.rows(), 0);
    }

    #[test]
    fn index() {
        let mut grid = Grid::new(3, 3);

        grid[(0, 0)] = 1;
        grid[(1, 2)] = 2;
        grid[(2, 1)] = 3;

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(1, 2)], 2);
        assert_eq!(grid[(2, 1)], 3);
    }

    #[test]
    fn iter() {
        let mut grid = Grid::new(3, 3);

        for ((col, row), i) in &mut grid {
            *i = (col, row);
        }

        assert_eq!(grid[(0, 0)], (0, 0));
        assert_eq!(grid[(1, 0)], (1, 0));
        assert_eq!(grid[(2, 0)], (2, 0));

        assert_eq!(grid[(0, 1)], (0, 1));
        assert_eq!(grid[(1, 1)], (1, 1));
        assert_eq!(grid[(2, 1)], (2, 1));

        assert_eq!(grid[(0, 2)], (0, 2));
        assert_eq!(grid[(1, 2)], (1, 2));
        assert_eq!(grid[(2, 2)], (2, 2));
    }
}

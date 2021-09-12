use crate::grid::Grid;

pub type UpdateNeighbor<R> = Option<fn(&mut <R as Ruleset>::NeighborData)>;

/// Defines cell state & next generation rules
pub trait Ruleset {
    type State: Default + Clone;
    type NeighborData: Default + Clone;

    /// Returns the state of a cell in the next generation based on the current state
    /// and the collected neighbor data
    fn next(s: &Self::State, n: &Self::NeighborData) -> Self::State;

    /// Returns a function that will be called for each of the cell's neighbors
    /// based on the previous and current state of the cell
    fn update_neighbor(prev: &Self::State, curr: &Self::State) -> UpdateNeighbor<Self>;
}

/// Stores cell state & runs rules
pub struct Automaton<R: Ruleset> {
    cells: [Grid<R::State>; 2],
    neighbor_data: [Grid<R::NeighborData>; 2],
}

impl<R: Ruleset> Automaton<R> {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cells: [
                Grid::new(cols, rows),
                Grid::new(cols, rows),
            ],
            neighbor_data: [
                Grid::new(cols + 2, rows + 2),
                Grid::new(cols + 2, rows + 2),
            ],
        }
    }

    /// Advances the grid to the next generation
    pub fn step(&mut self) {
        if let ([a], [b]) = self.neighbor_data.split_at_mut(1) {
            a.clone_from(b);
        } else {
            unreachable!()
        }

        for (col, row) in self.cells[0].indices() {
            let current = &self.cells[0][(col, row)];
            let neighbor_data = &self.neighbor_data[0][(col + 1, row + 1)];

            let next = R::next(current, neighbor_data);

            if let Some(update_neighbor) = R::update_neighbor(current, &next) {
                self.update_neighbors(col, row, update_neighbor);
            }

            self.cells[1][(col, row)] = next;
        }

        if let ([a], [b]) = self.cells.split_at_mut(1) {
            a.swap(b);
        } else {
            unreachable!()
        }
    }

    /// Returns the current cell state at the specified coordinates
    pub fn cell(&self, col: usize, row: usize) -> &R::State {
        &self.cells[0][(col, row)]
    }

    /// Returns a mutable reference to the current cell state at the specified coordinates
    pub fn with_cell_mut<F, O>(&mut self, col: usize, row: usize, f: F) -> O
    where
        F: FnOnce(&mut R::State) -> O,
    {
        let prev = self.cells[0][(col, row)].clone();
        let result = f(&mut self.cells[0][(col, row)]);

        if let Some(update_neighbor) = R::update_neighbor(&prev, &self.cells[0][(col, row)]) {
            self.update_neighbors(col, row, update_neighbor);
        }

        result
    }

    /// Sets the cell state at the specified coordinates and updates its neighbors
    /// using the given function
    pub fn set_cell(&mut self, col: usize, row: usize, new: R::State) {
        if col >= self.cols() || row >= self.rows() { return }

        let current = &self.cells[0][(col, row)];

        if let Some(update_neighbor) = R::update_neighbor(current, &new) {
            self.update_neighbors(col, row, update_neighbor);
        }

        self.cells[0][(col, row)] = new;
    }

    /// Copies the given pattern onto the grid in the specified location
    pub fn put(&mut self, pattern: &Grid<R::State>, left: isize, top: isize) {
        for ((col, row), cell) in pattern {
            let col = left + col as isize;
            let row = top + row as isize;

            if col >= 0 && row >= 0 {
                self.set_cell(col as usize, row as usize, cell.clone());
            }
        }
    }

    pub fn cols(&self) -> usize { self.cells[0].cols() }
    pub fn rows(&self) -> usize { self.cells[0].rows() }

    pub fn cells(&self) -> &Grid<R::State> { &self.cells[0] }

    fn update_neighbors(&mut self, col: usize, row: usize, update: fn(&mut R::NeighborData)) {
        update(&mut self.neighbor_data[1][(col,     row)]);
        update(&mut self.neighbor_data[1][(col + 1, row)]);
        update(&mut self.neighbor_data[1][(col + 2, row)]);

        update(&mut self.neighbor_data[1][(col,     row + 1)]);
        update(&mut self.neighbor_data[1][(col + 2, row + 1)]);

        update(&mut self.neighbor_data[1][(col,     row + 2)]);
        update(&mut self.neighbor_data[1][(col + 1, row + 2)]);
        update(&mut self.neighbor_data[1][(col + 2, row + 2)]);
    }

    pub fn clear(&mut self) {
        self.cells[0].clear();
        self.cells[1].clear();
        self.neighbor_data[0].clear();
        self.neighbor_data[1].clear();
    }
}

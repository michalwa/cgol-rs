use crate::grid::Grid;

pub type UpdateNeighbor<R> = Option<fn(&mut <R as Ruleset>::NeighborData)>;

/// Defines cell state & next generation rules
pub trait Ruleset {
    type State: Default + Clone;
    type NeighborData: Default + Clone;
    fn next(s: &Self::State, n: &Self::NeighborData) -> (Self::State, UpdateNeighbor<Self>);
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

            let (next, update_neighbor) = R::next(current, neighbor_data);
            self.cells[1][(col, row)] = next;

            if let Some(update_neighbor) = update_neighbor {
                self.update_neighbors(col, row, update_neighbor);
            }
        }

        if let ([a], [b]) = self.cells.split_at_mut(1) {
            a.swap(b);
        } else {
            unreachable!()
        }
    }

    /// Sets the cell state at the specified coordinates and updates its neighbors
    /// using the given function
    pub fn set(
        &mut self,
        col: usize,
        row: usize,
        new: R::State,
        update_neighbor: UpdateNeighbor<R>,
    ) {
        self.cells[0][(col, row)] = new;

        if let Some(update_neighbor) = update_neighbor.into() {
            self.update_neighbors(col, row, update_neighbor);
        }
    }

    /// Returns current cell state at the specified coordinates
    pub fn get(&self, col: usize, row: usize) -> &R::State {
        &self.cells[0][(col, row)]
    }

    pub fn cols(&self) -> usize { self.cells[0].cols() }
    pub fn rows(&self) -> usize { self.cells[0].rows() }
    pub fn grid(&self) -> &Grid<R::State> { &self.cells[0] }

    fn update_neighbors(&mut self, col: usize, row: usize, update: fn(&mut R::NeighborData)) {
        for n_col in col..(col + 3) {
            for n_row in row..(row + 3) {
                if !(n_col == col + 1 && n_row == row + 1) {
                    update(&mut self.neighbor_data[1][(n_col, n_row)]);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.cells[0].clear();
        self.cells[1].clear();
        self.neighbor_data[0].clear();
        self.neighbor_data[1].clear();
    }
}

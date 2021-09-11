use std::{
    cmp::{max, min},
    fmt::{self, Write},
    mem, thread,
    time::Duration,
};

trait Ruleset {
    type State: Default + Clone;
    type NeighborData: Default + Clone;
    fn next(s: &Self::State, n: &Self::NeighborData) -> (Self::State, fn(&mut Self::NeighborData));
}

struct Automaton<R: Ruleset> {
    cols: usize,
    rows: usize,
    cells: [Box<[R::State]>; 2],
    neighbor_data: Box<[R::NeighborData]>,
}

impl<R: Ruleset> Automaton<R> {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: [
                vec![Default::default(); cols * rows].into(),
                vec![Default::default(); cols * rows].into(),
            ],
            neighbor_data: vec![Default::default(); cols * rows].into(),
        }
    }

    fn step(&mut self) {
        for col in 0..self.cols {
            for row in 0..self.rows {
                let current = &self.cells[0][row * self.cols + col];
                let neighbor_data = &self.neighbor_data[row * self.cols + col];

                let (next, update_neighbor) = R::next(current, neighbor_data);
                self.cells[1][row * self.cols + col] = next;

                self.update_neighbors(col, row, update_neighbor);
            }
        }

        if let ([a], [b]) = self.cells.split_at_mut(1) {
            mem::swap(a, b);
        } else {
            unreachable!()
        }
    }

    fn update_neighbors(&mut self, col: usize, row: usize, update: fn(&mut R::NeighborData)) {
        let start_col = max(0, col as isize - 1) as usize;
        let end_col = min(self.cols, col + 2);
        let start_row = max(0, row as isize - 1) as usize;
        let end_row = min(self.rows, row + 2);

        for n_col in start_col..end_col {
            for n_row in start_row..end_row {
                if !(n_col == col && n_row == row) {
                    update(&mut self.neighbor_data[n_row * self.cols + n_col]);
                }
            }
        }
    }

    fn set(
        &mut self,
        col: usize,
        row: usize,
        new: R::State,
        update_neighbor: fn(&mut R::NeighborData),
    ) {
        self.cells[0][row * self.cols + col] = new;
        self.update_neighbors(col, row, update_neighbor);
    }
}

impl<R: Ruleset> fmt::Display for Automaton<R>
where
    R::State: fmt::Display,
    R::NeighborData: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.cells[0][row * self.cols + col].fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

struct Cgol;

impl Ruleset for Cgol {
    type State = ConwayCell;
    type NeighborData = u8;

    fn next(s: &Self::State, n: &Self::NeighborData) -> (Self::State, fn(&mut Self::NeighborData)) {
        match (s, n) {
            // Any live cell with two or three live neighbours survives
            (&ConwayCell::Live, 2..=3) => (ConwayCell::Live, |_| {}),
            // Any dead cell with three live neighbours becomes a live cell
            (&ConwayCell::Dead, &n) if n == 3 => (ConwayCell::Live, |n| *n += 1),
            // All other live cells die in the next generation
            (&ConwayCell::Live, _) => (ConwayCell::Dead, |n| *n -= 1),
            // Similarly, all other dead cells stay dead
            _ => (ConwayCell::Dead, |_| {}),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConwayCell {
    Dead,
    Live,
}

impl Default for ConwayCell {
    fn default() -> Self {
        Self::Dead
    }
}

impl fmt::Display for ConwayCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(match self {
            ConwayCell::Dead => '.',
            ConwayCell::Live => '@',
        })
    }
}

fn clear_terminal() {
    print!("{0}[2J{0}[H", 27 as char);
}

fn main() {
    let mut gol = Automaton::<Cgol>::new(60, 30);

    // Glider
    // gol.set(5, 5, ConwayCell::Live, |n| *n += 1);
    // gol.set(5, 6, ConwayCell::Live, |n| *n += 1);
    // gol.set(6, 5, ConwayCell::Live, |n| *n += 1);
    // gol.set(6, 6, ConwayCell::Live, |n| *n += 1);

    // Blinker
    gol.set(5, 5, ConwayCell::Live, |n| *n += 1);
    gol.set(5, 6, ConwayCell::Live, |n| *n += 1);
    gol.set(5, 7, ConwayCell::Live, |n| *n += 1);

    loop {
        clear_terminal();
        print!("{}", gol);
        thread::sleep(Duration::from_millis(500));
        gol.step();
    }
}

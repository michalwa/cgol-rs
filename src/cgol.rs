use crate::automaton::{Ruleset, UpdateNeighbor};

/// Conway's Game of Life ruleset
pub struct Cgol;

impl Ruleset for Cgol {
    type State = CgolCell;
    type NeighborData = u8;

    fn next(s: &Self::State, n: &Self::NeighborData) -> Self::State {
        match (s, n) {
            // Any live cell with two or three live neighbours survives
            (&CgolCell::Live(age), 2..=3) => CgolCell::Live(age.saturating_add(1)),
            // Any dead cell with three live neighbours becomes a live cell
            (&CgolCell::Dead, &n) if n == 3 => CgolCell::Live(0),
            // All other live cells die in the next generation
            (&CgolCell::Live(_), _) => CgolCell::Dead,
            // Similarly, all other dead cells stay dead
            (&CgolCell::Dead, _) => CgolCell::Dead,
        }
    }

    fn update_neighbor(prev: &Self::State, curr: &Self::State) -> UpdateNeighbor<Self> {
        match (prev, curr) {
            (&CgolCell::Live(_), &CgolCell::Dead) => Some(|n| *n -= 1),
            (&CgolCell::Dead, &CgolCell::Live(_)) => Some(|n| *n += 1),
            _ => None,
        }
    }
}

/// Conway's Game of Life cell state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgolCell {
    Dead,
    Live(u8),
}

impl CgolCell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Self::Dead => Self::Live(0),
            Self::Live(_) => Self::Dead,
        };
    }
}

impl Default for CgolCell {
    fn default() -> Self { Self::Dead }
}

pub mod patterns {
    use super::*;
    use crate::grid::Grid;
    use lazy_static::lazy_static;

    fn from_slice(cols: usize, rows: usize, pattern: &[u8]) -> Grid<CgolCell> {
        Grid::from_slice(cols, rows, pattern
            .iter()
            .map(|n| match n {
                0 => CgolCell::Dead,
                _ => CgolCell::Live(0),
            })
            .collect::<Vec<_>>()
            .as_ref()
        )
    }

    lazy_static! {
        // Still lifes
        pub static ref BLOCK_1: Grid<CgolCell> = from_slice(1, 1, &[1]);
        pub static ref BLOCK_2: Grid<CgolCell> = from_slice(2, 2, &[1, 1, 1, 1]);

        pub static ref BEEHIVE: Grid<CgolCell> =
            from_slice(4, 3, &[
                0, 1, 1, 0,
                1, 0, 0, 1,
                0, 1, 1, 0,
            ]);

        pub static ref LOAF: Grid<CgolCell> =
            from_slice(4, 4, &[
                0, 1, 1, 0,
                1, 0, 0, 1,
                0, 1, 0, 1,
                0, 0, 1, 0,
            ]);

        pub static ref BOAT: Grid<CgolCell> =
            from_slice(3, 3, &[
                1, 1, 0,
                1, 0, 1,
                0, 1, 0,
            ]);

        pub static ref TUB: Grid<CgolCell> =
            from_slice(3, 3, &[
                0, 1, 0,
                1, 0, 1,
                0, 1, 0,
            ]);

        // Oscillators
        pub static ref BLINKER: Grid<CgolCell> = from_slice(1, 3, &[1, 1, 1]);

        pub static ref TOAD: Grid<CgolCell> =
            from_slice(4, 2, &[
                0, 1, 1, 1,
                1, 1, 1, 0,
            ]);

        pub static ref BEACON: Grid<CgolCell> =
            from_slice(4, 4, &[
                1, 1, 0, 0,
                1, 1, 0, 0,
                0, 0, 1, 1,
                0, 0, 1, 1,
            ]);

        pub static ref PULSAR: Grid<CgolCell> =
            from_slice(13, 13, &[
                0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0,
            ]);

        // Spaceships
        pub static ref GLIDER: Grid<CgolCell> =
            from_slice(3, 3, &[
                0, 1, 0,
                0, 0, 1,
                1, 1, 1,
            ]);

        pub static ref LWSS: Grid<CgolCell> =
            from_slice(5, 4, &[
                1, 0, 0, 1, 0,
                0, 0, 0, 0, 1,
                1, 0, 0, 0, 1,
                0, 1, 1, 1, 1,
            ]);

        pub static ref MWSS: Grid<CgolCell> =
            from_slice(6, 5, &[
                0, 0, 1, 0, 0, 0,
                1, 0, 0, 0, 1, 0,
                0, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 1,
                0, 1, 1, 1, 1, 1,
            ]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::Automaton;

    #[test]
    fn square() {
        let mut cgol = Automaton::<Cgol>::new([5, 5]);

        cgol.set_cell(1, 1, CgolCell::Live(0));
        cgol.set_cell(1, 2, CgolCell::Live(0));
        cgol.set_cell(2, 1, CgolCell::Live(0));
        cgol.set_cell(2, 2, CgolCell::Live(0));

        let mut initial = cgol.cells().clone();

        cgol.step();

        for (_, c) in &mut initial {
            if let CgolCell::Live(n) = c { *n += 1; }
        }
        assert_eq!(cgol.cells(), &initial);
    }

    #[test]
    fn blinker() {
        let mut cgol = Automaton::<Cgol>::new([5, 5]);
        cgol.set_cell(1, 2, CgolCell::Live(0));
        cgol.set_cell(2, 2, CgolCell::Live(0));
        cgol.set_cell(3, 2, CgolCell::Live(0));
        let mut state = cgol.cells().clone();

        cgol.step();
        state[(1, 2)] = CgolCell::Dead;
        state[(3, 2)] = CgolCell::Dead;
        state[(2, 1)] = CgolCell::Live(0);
        state[(2, 2)] = CgolCell::Live(1);
        state[(2, 3)] = CgolCell::Live(0);

        assert_eq!(cgol.cells(), &state);
    }
}

use std::fmt::{self, Write};
use crate::automaton::Ruleset;

/// Conway's Game of Life ruleset
pub struct Cgol;

impl Ruleset for Cgol {
    type State = CgolCell;
    type NeighborData = u8;

    fn next(s: &Self::State, n: &Self::NeighborData) -> (Self::State, fn(&mut Self::NeighborData)) {
        match (s, n) {
            // Any live cell with two or three live neighbours survives
            (&CgolCell::Live, 2..=3) => (CgolCell::Live, |_| {}),
            // Any dead cell with three live neighbours becomes a live cell
            (&CgolCell::Dead, &n) if n == 3 => (CgolCell::Live, |n| *n += 1),
            // All other live cells die in the next generation
            (&CgolCell::Live, _) => (CgolCell::Dead, |n| *n -= 1),
            // Similarly, all other dead cells stay dead
            (&CgolCell::Dead, _) => (CgolCell::Dead, |_| {}),
        }
    }
}

/// Conway's Game of Life cell state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CgolCell {
    Dead,
    Live,
}

impl Default for CgolCell {
    fn default() -> Self { Self::Dead }
}

impl fmt::Display for CgolCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(match self {
            CgolCell::Dead => '.',
            CgolCell::Live => '@',
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::Automaton;

    #[test]
    fn square() {
        let mut cgol = Automaton::<Cgol>::new(5, 5);

        cgol.set(1, 1, CgolCell::Live, |n| *n += 1);
        cgol.set(1, 2, CgolCell::Live, |n| *n += 1);
        cgol.set(2, 1, CgolCell::Live, |n| *n += 1);
        cgol.set(2, 2, CgolCell::Live, |n| *n += 1);

        let initial = cgol.grid().clone();

        cgol.step();

        assert_eq!(cgol.grid(), &initial);
    }

    #[test]
    fn blinker() {
        let mut cgol = Automaton::<Cgol>::new(5, 5);
        cgol.set(1, 2, CgolCell::Live, |n| *n += 1);
        cgol.set(2, 2, CgolCell::Live, |n| *n += 1);
        cgol.set(3, 2, CgolCell::Live, |n| *n += 1);
        let mut state = cgol.grid().clone();

        cgol.step();
        state[(1, 2)] = CgolCell::Dead;
        state[(3, 2)] = CgolCell::Dead;
        state[(2, 1)] = CgolCell::Live;
        state[(2, 3)] = CgolCell::Live;

        assert_eq!(cgol.grid(), &state);
    }
}

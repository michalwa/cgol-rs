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

impl Default for CgolCell {
    fn default() -> Self { Self::Dead }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::automaton::Automaton;

    #[test]
    fn square() {
        let mut cgol = Automaton::<Cgol>::new(5, 5);

        cgol.set(1, 1, CgolCell::Live(0));
        cgol.set(1, 2, CgolCell::Live(0));
        cgol.set(2, 1, CgolCell::Live(0));
        cgol.set(2, 2, CgolCell::Live(0));

        let mut initial = cgol.cells().clone();

        cgol.step();

        for (_, c) in &mut initial {
            if let CgolCell::Live(n) = c { *n += 1; }
        }
        assert_eq!(cgol.cells(), &initial);
    }

    #[test]
    fn blinker() {
        let mut cgol = Automaton::<Cgol>::new(5, 5);
        cgol.set(1, 2, CgolCell::Live(0));
        cgol.set(2, 2, CgolCell::Live(0));
        cgol.set(3, 2, CgolCell::Live(0));
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

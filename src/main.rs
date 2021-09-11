use std::{thread, time::Duration};
use seagull::{Automaton, Cgol, cgol::CgolCell};

fn main() {
    let mut gol = Automaton::<Cgol>::new(5, 5);

    // Glider
    // gol.set(5, 5, ConwayCell::Live, |n| *n += 1);
    // gol.set(5, 6, ConwayCell::Live, |n| *n += 1);
    // gol.set(6, 5, ConwayCell::Live, |n| *n += 1);
    // gol.set(6, 6, ConwayCell::Live, |n| *n += 1);

    // Blinker
    gol.set(2, 1, CgolCell::Live, |n| *n += 1);
    gol.set(2, 2, CgolCell::Live, |n| *n += 1);
    gol.set(2, 3, CgolCell::Live, |n| *n += 1);

    loop {
        clear_terminal();
        print!("{}", gol);
        thread::sleep(Duration::from_millis(500));
        gol.step();
    }
}

fn clear_terminal() {
    print!("{0}[2J{0}[H", 27 as char);
}

use piston_window::*;
use seagull::{cgol::CgolCell, Automaton, Cgol};
use std::time::{Duration, Instant};

fn main() {
    let cell_size: f64 = 4.0;
    let mut cgol = Automaton::<Cgol>::new(200, 200);

    // Random
    use rand::random;
    for (col, row) in cgol.grid().indices() {
        if random::<bool>() {
            cgol.set(col, row, CgolCell::Live(0), |n| *n += 1);
        }
    }

    // Glider
    // cgol.set(5, 5, CgolCell::Live(0), |n| *n += 1);
    // cgol.set(5, 6, CgolCell::Live(0), |n| *n += 1);
    // cgol.set(5, 7, CgolCell::Live(0), |n| *n += 1);
    // cgol.set(4, 7, CgolCell::Live(0), |n| *n += 1);
    // cgol.set(3, 6, CgolCell::Live(0), |n| *n += 1);

    // Blinker
    // gol.set(2, 1, CgolCell::Live, |n| *n += 1);
    // gol.set(2, 2, CgolCell::Live, |n| *n += 1);
    // gol.set(2, 3, CgolCell::Live, |n| *n += 1);

    let mut window: PistonWindow = WindowSettings::new(
        "Conway's Game of Life",
        [
            cgol.grid().cols() as f64 * cell_size,
            cgol.grid().rows() as f64 * cell_size,
        ],
    )
    .resizable(false)
    .build()
    .unwrap();

    let update_interval = Duration::from_millis(50);
    let mut last_update = Instant::now();
    let mut running = false;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for ((col, row), state) in cgol.grid() {
                if let &CgolCell::Live(age) = state {
                    let lightness = 1.0 / age.saturating_add(1) as f32;
                    rectangle(
                        [lightness, lightness, lightness, 1.0],
                        [col as f64 * cell_size, row as f64 * cell_size, cell_size, cell_size],
                        c.transform,
                        g,
                    );
                }
            }
        });

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => running = !running,
                _ => (),
            }
        }

        if running {
            let now = Instant::now();
            if now - last_update >= update_interval {
                last_update = now;
                cgol.step();
            }
        }
    }
}

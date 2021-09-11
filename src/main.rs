use piston_window::*;
use seagull::{cgol::CgolCell, Automaton, Cgol};
use std::time::{Duration, Instant};

fn main() {
    let cell_size: f64 = 15.0;
    let mut cgol = Automaton::<Cgol>::new(50, 50);

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
    let mut mouse_pos = [0f64; 2];

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            for ((col, row), state) in cgol.grid() {
                if let &CgolCell::Live(age) = state {
                    let lightness = 1.0 / age.saturating_add(1) as f32;
                    rectangle(
                        [lightness, lightness, lightness, if running { 1.0 } else { 0.05 }],
                        [col as f64 * cell_size, row as f64 * cell_size, cell_size, cell_size],
                        c.transform,
                        g,
                    );
                }
            }

            let cursor_col = (mouse_pos[0] / cell_size) as usize;
            let cursor_row = (mouse_pos[1] / cell_size) as usize;
            rectangle(
                [1.0, 1.0, 0.0, 0.5],
                [cursor_col as f64 * cell_size, cursor_row as f64 * cell_size, cell_size, cell_size],
                c.transform,
                g
            );
        });

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => running = !running,
                Button::Keyboard(Key::C) => cgol.clear(),
                Button::Keyboard(Key::R) => {
                    use rand::random;
                    cgol.clear();
                    for (col, row) in cgol.grid().indices() {
                        if random::<bool>() {
                            cgol.set(col, row, CgolCell::Live(0), Some(|n| *n += 1));
                        }
                    }
                }
                Button::Mouse(MouseButton::Left) => {
                    let col = (mouse_pos[0] / cell_size) as usize;
                    let row = (mouse_pos[1] / cell_size) as usize;

                    if cgol.get(col, row) == &CgolCell::Dead {
                        cgol.set(col, row, CgolCell::Live(0), Some(|n| *n += 1));
                    } else {
                        cgol.set(col, row, CgolCell::Dead, Some(|n| *n -= 1));
                    }
                }
                _ => (),
            }
        }

        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
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

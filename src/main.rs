use piston_window::*;
use seagull::{
    cgol::{self, CgolCell},
    grid::Grid,
    Automaton, Cgol,
};
use std::time::{Duration, Instant};

fn main() {
    let brushes: &[&Grid<CgolCell>] = &[&cgol::patterns::BLOCK_1, &cgol::patterns::GLIDER];

    let cell_size: f64 = 8.0;
    let mut cgol = Automaton::<Cgol>::new(100, 100);

    let mut window: PistonWindow = WindowSettings::new(
        "Conway's Game of Life",
        [
            cgol.cells().cols() as f64 * cell_size,
            cgol.cells().rows() as f64 * cell_size,
        ],
    )
    .resizable(false)
    .build()
    .unwrap();

    let font_data = include_bytes!("../res/CONSOLA.TTF");
    let texture_ctx = window.create_texture_context();
    let mut font = Glyphs::from_bytes(font_data, texture_ctx, TextureSettings::new()).unwrap();

    let mut step_millis = 64;
    let mut last_update = Instant::now();
    let mut running = false;
    let mut cursor = [0usize; 2];
    let mut brush_idx: usize = 0;
    let mut show_age = true;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw cells
            for ((col, row), state) in cgol.cells() {
                if let &CgolCell::Live(age) = state {
                    let lightness = if show_age {
                        0.1f32.max(1.0 / age.saturating_add(1) as f32)
                    } else {
                        1.0
                    };

                    rectangle(
                        [lightness, lightness, lightness, 1.0],
                        [
                            col as f64 * cell_size,
                            row as f64 * cell_size,
                            cell_size,
                            cell_size,
                        ],
                        c.transform,
                        g,
                    );
                }
            }

            // Draw brush
            for ((col, row), cell) in brushes[brush_idx] {
                if let &CgolCell::Live(_) = cell {
                    let col =
                        cursor[0] as isize + col as isize - brushes[brush_idx].cols() as isize / 2;

                    let row =
                        cursor[1] as isize + row as isize - brushes[brush_idx].rows() as isize / 2;

                    rectangle(
                        [1.0, 1.0, 0.0, 0.2],
                        [
                            col as f64 * cell_size,
                            row as f64 * cell_size,
                            cell_size,
                            cell_size,
                        ],
                        c.transform,
                        g,
                    );
                }
            }

            // Draw info
            let info = format!(
                "{}\nstep: {}ms\nbrush: {}\nshow age: {:?}",
                if running { "running" } else { "paused" },
                step_millis,
                brush_idx,
                show_age,
            );

            for (i, line) in info.lines().enumerate() {
                text(
                    [0.6, 0.7, 1.0, 0.3], 14, line,
                    &mut font, c.transform.trans(10.0, (i + 1) as f64 * 20.0), g,
                )
                .unwrap();
            }

            font.factory.encoder.flush(device);
        });

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => running = !running,
                Button::Keyboard(Key::C) => cgol.clear(),
                Button::Keyboard(Key::A) => show_age = !show_age,
                Button::Keyboard(Key::Up) => step_millis = (step_millis / 2).max(16),
                Button::Keyboard(Key::Down) => step_millis = (step_millis * 2).min(2048),
                Button::Keyboard(Key::R) => {
                    use rand::random;
                    cgol.clear();
                    for (col, row) in cgol.cells().indices() {
                        if random::<bool>() {
                            cgol.set_cell(col, row, CgolCell::Live(0));
                        }
                    }
                }
                Button::Keyboard(Key::B) => brush_idx = (brush_idx + 1) % brushes.len(),
                Button::Mouse(MouseButton::Left) => {
                    let col = cursor[0] as isize - brushes[brush_idx].cols() as isize / 2;
                    let row = cursor[1] as isize - brushes[brush_idx].rows() as isize / 2;

                    if brush_idx == 0 {
                        cgol.with_cell_mut(col as usize, row as usize, |cell| cell.toggle());
                    } else {
                        cgol.put(brushes[brush_idx], col, row);
                    }
                }
                _ => (),
            }
        }

        if let Some([x, y]) = event.mouse_cursor_args() {
            cursor = [(x / cell_size) as usize, (y / cell_size) as usize];
        }

        if running {
            let now = Instant::now();
            if now - last_update >= Duration::from_millis(step_millis) {
                last_update = now;
                cgol.step();
            }
        }
    }
}

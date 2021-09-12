use piston_window::*;
use seagull::{
    cgol::{self, CgolCell},
    grid::Grid,
    Automaton, Cgol,
};
use std::time::{Duration, Instant};

type Brush<'a> = (&'static str, &'a Grid<CgolCell>);

fn main() {
    let brushes: &[Brush] = &[
        ("1x1", &cgol::patterns::BLOCK_1),
        ("glider", &cgol::patterns::GLIDER),
    ];

    let cell_size: f64 = 4.0;
    let mut cgol = Automaton::<Cgol>::new(200, 200);

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

    let min_step_millis = 16;
    let max_step_millis = 1024;

    let mut generation = 0u32;
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
            let brush = brushes[brush_idx].1;
            for ((col, row), cell) in brush {
                if let &CgolCell::Live(_) = cell {
                    let col =
                        cursor[0] as isize + col as isize - brush.cols() as isize / 2;

                    let row =
                        cursor[1] as isize + row as isize - brush.rows() as isize / 2;

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
                concat!(
                    "{:<9}[Space]\n",
                    "step     [Up/Down]: {}ms\n",
                    "brush    [B]:       {}\n",
                    "show age [A]:       {:?}\n",
                    "\n",
                    "generation: {}\n",
                ),
                if running { "running" } else { "paused" },
                step_millis,
                brushes[brush_idx].0,
                show_age,
                generation,
            );

            for (i, line) in info.lines().enumerate() {
                text(
                    [0.6, 0.7, 1.0, 1.0], 10, line,
                    &mut font, c.transform.trans(10.0, (i + 1) as f64 * 14.0 + 10.0), g,
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
                Button::Keyboard(Key::Up) => step_millis = (step_millis / 2).max(min_step_millis),
                Button::Keyboard(Key::Down) => step_millis = (step_millis * 2).min(max_step_millis),
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
                    let brush = brushes[brush_idx].1;
                    let col = cursor[0] as isize - brush.cols() as isize / 2;
                    let row = cursor[1] as isize - brush.rows() as isize / 2;

                    if brush_idx == 0 {
                        cgol.with_cell_mut(col as usize, row as usize, |cell| cell.toggle());
                    } else {
                        cgol.put(brush, col, row);
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
                generation += 1;
            }
        }
    }
}

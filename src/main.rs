use piston_window::*;
use seagull::{
    cgol::{self, CgolCell},
    grid::Grid,
    Automaton, Cgol,
};
use std::time::{Duration, Instant};

mod renderer;
use renderer::Renderer;

mod utils;
use utils::RangeExt;

type Brush<'a> = (&'static str, &'a Grid<CgolCell>);

fn main() {
    let brushes: &[Brush] = &[
        ("1x1", &cgol::patterns::BLOCK_1),
        ("2x2", &cgol::patterns::BLOCK_2),
        ("beehive", &cgol::patterns::BEEHIVE),
        ("loaf", &cgol::patterns::LOAF),
        ("boat", &cgol::patterns::BOAT),
        ("tub", &cgol::patterns::TUB),

        ("blinker", &cgol::patterns::BLINKER),
        ("toad", &cgol::patterns::TOAD),
        ("beacon", &cgol::patterns::BEACON),
        ("pulsar", &cgol::patterns::PULSAR),

        ("glider", &cgol::patterns::GLIDER),
        ("light-weight spaceship", &cgol::patterns::LWSS),
        ("middle-weight spaceship", &cgol::patterns::MWSS),
    ];

    let dims = [200, 200];
    let mut cgol = Automaton::<Cgol>::new(dims);

    let mut renderer = Renderer {
        cell_size: 4.0,
        show_age: true,
    };

    let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life", renderer.window_size(dims))
            .resizable(false)
            .build()
            .unwrap();

    let font_data = include_bytes!("../res/CONSOLA.TTF");
    let texture_ctx = window.create_texture_context();
    let mut font = Glyphs::from_bytes(font_data, texture_ctx, TextureSettings::new()).unwrap();

    let mut running = false;
    let mut cursor = [0usize; 2];
    let mut last_update = Instant::now();
    let mut generation = 0u32;

    let mut step_millis = 64;
    let step_millis_range = 16..=1024;
    let mut brush_idx: usize = 0;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            renderer.draw_grid(cgol.cells(), c, g);
            renderer.draw_brush(brushes[brush_idx].1, cursor, c, g);

            // Draw info
            let info = format!(
                concat!(
                    "  [Space] {}\n",
                    "[Up/Down] step:     {}ms\n",
                    "      [B] brush:    {}\n",
                    "      [A] show age: {:?}\n",
                    "      [R] randomize\n",
                    "      [C] clear\n",
                    "\n",
                    "generation: {}\n",
                ),
                if running { "running" } else { "paused" },
                step_millis,
                brushes[brush_idx].0,
                renderer.show_age,
                generation,
            );

            let text_color = [0.6, 0.7, 1.0, 1.0];
            for (i, line) in info.lines().enumerate() {
                text(
                    text_color, 10, line, &mut font,
                    c.transform.trans(10.0, (i + 1) as f64 * 14.0 + 10.0),
                    g,
                )
                .unwrap();
            }

            font.factory.encoder.flush(device);
        });

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => running = !running,
                Button::Keyboard(Key::C) => cgol.clear(),
                Button::Keyboard(Key::A) => renderer.show_age = !renderer.show_age,
                Button::Keyboard(Key::Up) => step_millis = step_millis_range.clamp(step_millis / 2),
                Button::Keyboard(Key::Down) => step_millis = step_millis_range.clamp(step_millis * 2),
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

        if let Some(pos) = event.mouse_cursor_args() {
            cursor = renderer.pos_to_indices(pos);
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

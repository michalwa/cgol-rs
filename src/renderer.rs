use piston_window::*;
use seagull::{cgol::CgolCell, grid::Grid};

pub struct Renderer {
    pub cell_size: f64,
    pub show_age: bool,
}

impl Renderer {
    pub fn draw_grid(&self, grid: &Grid<CgolCell>, c: Context, g: &mut impl Graphics) {
        for ((col, row), state) in grid {
            if let &CgolCell::Live(age) = state {
                let lightness = if self.show_age {
                    0.1f32.max(1.0 / age.saturating_add(1) as f32)
                } else {
                    1.0
                };

                rectangle(
                    [lightness, lightness, lightness, 1.0],
                    [
                        col as f64 * self.cell_size,
                        row as f64 * self.cell_size,
                        self.cell_size,
                        self.cell_size,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }

    pub fn draw_brush(
        &self,
        brush: &Grid<CgolCell>,
        cursor: [usize; 2],
        c: Context,
        g: &mut impl Graphics,
    ) {
        for ((col, row), cell) in brush {
            if let &CgolCell::Live(_) = cell {
                let col = cursor[0] as isize + col as isize - brush.cols() as isize / 2;
                let row = cursor[1] as isize + row as isize - brush.rows() as isize / 2;

                rectangle(
                    [1.0, 1.0, 0.0, 0.2],
                    [
                        col as f64 * self.cell_size,
                        row as f64 * self.cell_size,
                        self.cell_size,
                        self.cell_size,
                    ],
                    c.transform,
                    g,
                );
            }
        }
    }

    pub fn pos_to_indices(&self, pos: [f64; 2]) -> [usize; 2] {
        [(pos[0] / self.cell_size) as usize, (pos[1] / self.cell_size) as usize]
    }

    pub fn window_size(&self, dims: [usize; 2]) -> [f64; 2] {
        [dims[0] as f64 * self.cell_size, dims[1] as f64 * self.cell_size]
    }
}

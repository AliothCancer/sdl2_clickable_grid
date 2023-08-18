use sdl2::{pixels::Color, rect::Rect};
const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 0,
};

const BP_DIMENSION: u32 = 100; // square

pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub matrix: Vec<BigPixel>,
}

#[derive(Clone)]
pub struct BigPixel {
    pub rect: Rect,
    pub color: Color,
}

impl BigPixel {
    fn new(x: i32, y: i32) -> Self {
        BigPixel {
            rect: Rect::new(x, y, BP_DIMENSION, BP_DIMENSION),
            color: WHITE,
        }
    }
    pub fn set_color(&mut self) {
        self.color = GREEN;
    }
}

impl Grid {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let (cols, rows) = Self::calculate_matrix_dimension(screen_width, screen_height);

        let mut matrix = vec![];
        let gap = 0;
        for row in 0..rows {
            let row: i32 = row.try_into().unwrap();
            for col in 0..cols {
                let col: i32 = col.try_into().unwrap();
                matrix.push(BigPixel::new(
                    (row * BP_DIMENSION as i32) + gap,
                    (col * BP_DIMENSION as i32) + gap,
                ))
            }
        }

        Grid { cols, rows, matrix }
    }
    fn calculate_matrix_dimension(screen_width: u32, screen_height: u32) -> (u32, u32) {
        let cols = screen_width / BP_DIMENSION;
        let rows = screen_height / BP_DIMENSION;
        (cols, rows)
    }
}

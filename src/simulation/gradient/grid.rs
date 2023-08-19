use sdl2::{pixels::Color, rect::Rect};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 0,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub const BP_DIMENSION: u32 = 10; // square

pub struct Grid {
    pub cols: i32,
    pub rows: i32,
    pub matrix: Vec<Vec<BigPixel>>,
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
/*
[col1, col2,col3]
*/

impl Grid {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let (cols, rows) = Self::calculate_matrix_dimension(screen_width, screen_height);

        let mut matrix = vec![];
        let gap = 0;
        for row in 0..rows { 
            let mut column = vec![];
            for col in 0..cols {
                column.push(BigPixel::new(
                    (row * BP_DIMENSION as i32) + gap,
                    (col * BP_DIMENSION as i32) + gap,
                ))
            }
            matrix.push(column);
        }

        Grid { cols, rows, matrix }
    }
    fn calculate_matrix_dimension(screen_width: i32, screen_height: i32) -> (i32, i32) {
        let cols = screen_width / BP_DIMENSION as i32;
        let rows = screen_height / BP_DIMENSION as i32;
        (cols, rows)
    }

    pub fn get_mut_column(&mut self, index: i32)-> &mut Vec<BigPixel>{
        let index: usize = index.try_into().unwrap();
        self.matrix.get_mut(index).unwrap()
    }

}

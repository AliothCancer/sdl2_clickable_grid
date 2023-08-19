
pub mod grid;

use grid::Grid;

pub struct Simulation{
    grid: Grid
}

impl Simulation{
    pub fn new() -> Self{
        Simulation { grid: Grid {  matrix: vec![]} }
    }
}
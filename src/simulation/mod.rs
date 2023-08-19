pub mod gradient;

mod win;

//use rayon::prelude::*;
use std::time::Duration;

use sdl2::{
    event::Event,
    mouse::MouseButton,
    pixels::Color,
    EventPump,
};

use self::{
    gradient::grid::{Grid, GREEN, BP_DIMENSION, WHITE, BLACK},
    win::SimWindow,
};

const _BLACK: Color = Color::RGB(0, 0, 0);

#[derive(Clone, Copy)]
pub enum Message {
    Continue,
    Update(i32, i32),
    Stop,
}

#[derive(PartialEq)]
enum State {
    Initializing,
    Runnning,
    Stopped,
}

#[derive(Debug)]
struct ColoredBigPixel{
    col: i32,
    row: i32,
}
pub struct Simulation {
    state: State,
    sim_win: SimWindow,
    event_pump: EventPump,
    object: Grid,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation::new()
    }
}

impl Simulation {
    pub fn main_loop(&mut self) {
        self.sim_win.canvas.set_draw_color(BLACK);
        self.sim_win.canvas.clear();
        self.sim_win.canvas.present();

        //std::thread::sleep(Duration::new(5, 0));
        
        self.sim_win.canvas.set_draw_color(GREEN);
        while self.state != State::Stopped {

            
            //self.sim_win.canvas.clear();
            let message = self.event_handler();
            self.state_manager(message);
            //self.sim_win.canvas.present();
            
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
            
        }
    }

    pub fn new() -> Simulation {
        let state = State::Initializing;
        let sim_win = SimWindow::new();
        let event_pump = sim_win.sdl_context.event_pump().unwrap();
        let object = Grid::new(sim_win.width, sim_win.height);
        

        Simulation {
            state,
            sim_win,
            event_pump,
            object
        }
    }
    pub fn change_color(&mut self, x:i32, y:i32) {
        
        let (col_ind,row_ind) = Self::get_gridCoor_from_screenCoor(x, y);
        let rect = self.object.matrix[col_ind as usize][row_ind as usize].rect;
        
        self.sim_win.canvas.fill_rect(rect).unwrap();
        self.sim_win.canvas.present();
    }
    pub fn get_gridCoor_from_screenCoor(x: i32, y:i32) -> (i32, i32) {
        //let rem_col = x % BP_DIMENSION as i32; 
        //let rem_row = y % BP_DIMENSION as i32;
        let col = x / BP_DIMENSION as i32;
        let row = y / BP_DIMENSION as i32;

        

        (col,row)
    }

    pub fn state_manager(&mut self, message: Message) {
        match message {
            Message::Continue => self.state = State::Runnning,
            Message::Update(x, y) => self.change_color(x, y),
            Message::Stop => self.state = State::Stopped,
        }
    }

    pub fn event_handler(&mut self) -> Message {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Message::Stop,
                Event::MouseMotion {
                    x, y, mousestate, ..
                } if mousestate.is_mouse_button_pressed(MouseButton::Left) => {
                    return Message::Update(x, y)
                }
                //Event::MouseButtonDown { x, y, .. } => return Message::Update(x, y),
                _ => (),
            }
        }
        Message::Continue
    }
}

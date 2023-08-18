pub mod gradient;

mod win;

use rayon::prelude::*;
use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, EventPump};

use self::{gradient::grid::Grid, win::SimWindow};

const _BLACK: Color = Color::RGB(0, 0, 0);

enum Message {
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

pub struct Simulation {
    state: State,
    message: Message,
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
        while self.state != State::Stopped {
            self.event_handler();
            self.state_manager();
            //self.sim_win.canvas.set_draw_color(BLACK);
            self.sim_win.canvas.clear();
            self.draw();
            self.sim_win.canvas.present();

            //self.update();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn new() -> Simulation {
        let message = Message::Continue;
        let state = State::Initializing;
        let sim_win = SimWindow::new();
        let event_pump = sim_win.sdl_context.event_pump().unwrap();
        let object = Grid::new(sim_win.width, sim_win.height);

        Simulation {
            message,
            state,
            sim_win,
            event_pump,
            object,
        }
    }
    pub fn draw(&mut self) {
        self.object.matrix.iter().for_each(|b_pix| {
            self.sim_win.canvas.set_draw_color(b_pix.color);
            self.sim_win.canvas.fill_rect(b_pix.rect).unwrap();
        });
    }

    pub fn update(&mut self, x: i32, y: i32) {
        let point = Point::new(x, y);

        // using rayon
        self.object
            .matrix
            .par_iter_mut()
            .find_any(|r| r.rect.contains_point(point))
            .unwrap()
            .set_color();
    }

    pub fn state_manager(&mut self) {
        match self.message {
            Message::Continue => self.state = State::Runnning,
            Message::Update(x, y) => self.update(x, y),
            Message::Stop => self.state = State::Stopped,
        }
    }

    pub fn event_handler(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.message = Message::Stop,
                Event::MouseMotion {
                    x, y, mousestate, ..
                } if mousestate.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) => {
                    self.message = Message::Update(x, y)
                }
                _ => (), //println!("{event:?}")
            }
        }
    }
}

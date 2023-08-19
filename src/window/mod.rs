/*
It will manage sdl init as well as the sdl Window and Canvas<Window>
*/
mod colors;

use std::time::Duration;

use colors::*;
use sdl2::{render::Canvas, video::Window, Sdl, VideoSubsystem};
use crate::simulation::Simulation;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
pub struct SimWindow {
    title: String,
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub height: i32,
    pub width: i32,
    pub canvas: Canvas<Window>,
    simulation: Simulation,
    break_loop: bool
}

impl SimWindow {
    pub fn new(title: impl Into<String>, simulation: Simulation) -> SimWindow {
        let sdl_context = sdl2::init().unwrap();

        let title = title.into();
        
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&title, WIDTH as u32,HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        

        SimWindow {
            title,
            sdl_context,
            video_subsystem,
            height: HEIGHT,
            width: WIDTH,
            canvas,
            simulation,
            break_loop: false
        }
    }

    pub fn main_loop(&mut self){
        self.draw();

        while self.break_loop {
            
        } {
            std::thread::sleep(Duration::from_micros(1_000_000 / 60))
        }
    }

    fn draw(&mut self){
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        self.canvas.present();
    }

    fn event_handler(&mut self){
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => ,
                _ => (),
            }
        }
        Message::Continue
    }
}
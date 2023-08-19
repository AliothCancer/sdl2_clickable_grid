/*
It will manage sdl init as well as the sdl Window and Canvas<Window>
*/

use sdl2::{render::Canvas, video::Window, Sdl, VideoSubsystem};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;
pub struct SimWindow {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub height: i32,
    pub width: i32,
    pub canvas: Canvas<Window>,
}

impl SimWindow {
    pub fn new() -> SimWindow {
        let sdl_context = sdl2::init().unwrap();

        
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Grid of Rectangles", WIDTH as u32,HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        SimWindow {
            sdl_context,
            video_subsystem,
            height: HEIGHT,
            width: WIDTH,
            canvas,
        }
    }
}

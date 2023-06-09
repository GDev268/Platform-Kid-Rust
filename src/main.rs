extern crate sdl2;

use std::borrow::{Borrow, BorrowMut};

use sdl2::VideoSubsystem;
use::sdl2::image;
use::sdl2::video::Window;

mod graphics;

use graphics::Graphics;

fn main() {
    println!("Hello World!");
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
}

extern crate sdl2;

use std::borrow::{Borrow, BorrowMut};

use sdl2::VideoSubsystem;
use::sdl2::image;
use::sdl2::video::Window;

mod graphics;

use graphics::Graphics;


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::*;
use sdl2::image::*;

fn main() {
    println!("Hello World!");
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();


}



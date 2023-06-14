#[macro_use]
extern crate sdl2;
mod fps;
mod graphics;
mod platformtimer;

use std::borrow::{Borrow, BorrowMut};

use crate::sdl2::VideoSubsystem;
use crate::sdl2::image;
use crate::sdl2::libc::SYS_nanosleep;
use crate::sdl2::video::Window;

use crate::graphics::Graphics;


use crate::sdl2::pixels::Color;
use crate::sdl2::rect::Rect;
use crate::sdl2::render::{Canvas,Texture};
use crate::sdl2::surface::Surface;
use crate::sdl2::ttf::Font;
use crate::sdl2::*;
use crate::sdl2::image::*;

use crate::fps::PlatformFPS;

use std::time::{Instant,UNIX_EPOCH,Duration}; 
use std::thread::sleep;

pub static mut curTime:u128 = 0;

fn main() {
    let mut fpsManager:PlatformFPS = PlatformFPS::new();
    let mut globalTimer:Instant = Instant::now();

    println!("Hello World!");
    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    loop{
        unsafe {
            curTime = Instant::elapsed(&globalTimer).as_millis();  
            fpsManager.update();
            if fpsManager._fps > 5*1000/60 {
                sleep(Duration::from_millis(1000 / 60));
            }
            println!("Current FPS Count: {}",fpsManager._fps);
  
        }
    }
}
#[macro_use]
extern crate sdl2;
mod fps;
mod graphics;
mod platformtimer;
mod game;
mod sprite;

use std::borrow::{Borrow, BorrowMut};

/*use crate::sdl2::VideoSubsystem;
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
use crate::sdl2::image::*;*/

use crate::game::Game;

use std::time::{Instant,UNIX_EPOCH,Duration}; 
 
pub static mut curTime:u128 = 0;

fn main() {
    let mut game:Game = Game::new();
    game.gamesLoop();

}
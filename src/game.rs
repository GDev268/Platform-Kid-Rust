use crate::fps::PlatformFPS;
use crate::graphics::{Graphics, self};
use crate::sdl2::event::Event;
use std::time::{Instant,UNIX_EPOCH,Duration}; 
use sdl2::keyboard::Keycode; 


use crate::sdl2::VideoSubsystem;
use crate::sdl2::image::{InitFlag,SaveSurface};

use sdl2::pixels::Color;

use crate::curTime;

pub struct Game{
    fpsManager: PlatformFPS,
    MAX_FRAME_TIME:i32,
    _FPS:i32,
    running:bool,
    graphics:Graphics,
}

impl Game {
    pub fn new() -> Self{
        let mut fpsManager:PlatformFPS = PlatformFPS::new();
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
        
        let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string());
        
        let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .map_err(|e| e.to_string());
    
        let graphics:Graphics = Graphics::new("Platform", sdl_context, video_subsystem); 

        Self{
           fpsManager: PlatformFPS::new(),
           MAX_FRAME_TIME: 0,
           _FPS: 0,
           running: true,
           graphics,
        }
        
        
    }

    pub fn gamesLoop(&mut self){
        let mut event_pump = self.graphics.context.event_pump().unwrap();
        let mut globalTimer:Instant = Instant::now();

        self.graphics._renderer.set_draw_color(Color::RGB(42,253,5));
        self.graphics._renderer.clear();
        self.graphics._renderer.present();

        'running: loop{
            self.graphics._renderer.set_draw_color(Color::RGB(42,253,5));
            self.graphics._renderer.clear();
            unsafe {
                curTime = Instant::elapsed(&globalTimer).as_millis();  
            }

            for event in event_pump.poll_iter(){
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {break 'running},
                    _ => {}
                }
            }

            self.graphics._renderer.present();

            let surface = self.graphics.loadImage("ada");
        }

        
    }

}

/*        fpsManager.update();
        if fpsManager._fps > 5*1000/60 {
            sleep(Duration::from_millis(1000 / 60));
        }
        println!("Current FPS Count: {}",fpsManager._fps); */
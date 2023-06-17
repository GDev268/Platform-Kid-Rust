use std::time::{Instant,UNIX_EPOCH,Duration}; 

use crate::curTime;

pub struct PlatformFPS{
    pub _fps:u32,
    pub maxMillisec:i32,
    pub startTick:Instant,
    pub deltaTime:Duration,
    pub fpsLimit:Duration,
    pub frameCount:f64,
    pub frameElapsed:f64,
    pub elapsedStarted:Instant,
}

impl PlatformFPS{
    pub fn new() -> Self {
        Self{
            _fps: 0,
            maxMillisec: 0,
            startTick: Instant::now(),
            deltaTime: Duration::new(0,0),
            fpsLimit: Duration::new(0,0),
            frameCount:0f64,
            frameElapsed:0f64,
            elapsedStarted: Instant::now(),
        }
    }

    pub fn update(&mut self){
        self.frameCount += 1f64;
        self.frameElapsed = self.elapsedStarted.elapsed().as_secs() as f64 +
        (self.elapsedStarted.elapsed().subsec_nanos() as f64 / 1000000000f64);
        
        self.deltaTime = self.startTick.elapsed();
    }
}


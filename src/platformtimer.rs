use std::time::{SystemTime,UNIX_EPOCH};
use crate::curTime;

pub struct PlatformTimer{
    startTick: u128,
    endTick: u128,
    hasStarted: bool,
    hasPaused: bool
}

impl PlatformTimer{
    pub fn new() -> Self{
        return Self{
            startTick: 0,
            endTick: 0,
            hasStarted:false,
            hasPaused: false
        };
    }

    pub fn getTicks(&self) -> u128 {
        return self.endTick - self.startTick;
    } 

    pub fn start(&mut self) {
        self.hasStarted = true;
        self.hasPaused = false;
        
        self.startTick = unsafe {curTime};
    }

    pub fn pause(this: &mut Self){
        
    }

    pub fn resume(this: &mut Self){

    }

    pub fn stop(this: &mut Self){
        
    }

    //Check the timer
    pub fn isStarted(this: &Self) -> bool{
        return false;
    }

    pub fn isPaused(this: &Self) -> bool {
        return false;
    }
}
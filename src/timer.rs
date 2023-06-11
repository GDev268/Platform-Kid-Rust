pub struct Timer{
    startTick: u32,
    endTick: u32,
    hasStarted: bool,
    hasPaused: bool
}

impl Timer{
    pub fn new(){

    }

    pub fn getTicks(this: &Self) -> u32 {
        return -1;
    } 

    pub fn start(this: &mut Self) {

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
        return fasle;
    }
}
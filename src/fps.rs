use crate::curTime;

pub struct PlatformFPS{
    pub _fps:i32,
    pub maxMillisec:i32,
    pub startTick:u128,
    pub endTick:u128,
    pub fpsCount:i32,
}

impl PlatformFPS{
    pub fn new() -> Self {
        Self{
            _fps: 0,
            maxMillisec: 0,
            startTick: unsafe {curTime},
            endTick: 0,
            fpsCount: 0,
        }
    }

    pub fn update(&mut self){
        if self.endTick - self.startTick > 1000 {
            self._fps = self.fpsCount;
            self.fpsCount = 0;
            self.startTick = self.endTick;
        }
        self.endTick = unsafe {curTime};
        self.fpsCount += 1;
    }
}


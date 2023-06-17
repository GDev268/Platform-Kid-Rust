use crate::sdl2::rect::Rect;
use crate::sdl2::render::Texture;

pub struct Sprite{
    _sourceRect:Rect,
    _spriteSheet:Texture,
    _x:f32,
    _y:f32,
}

impl Sprite{
    pub fn new() -> Self{
        Self{}
    }
}
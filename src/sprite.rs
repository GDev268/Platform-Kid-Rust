use std::ops::Deref;

use crate::graphics::Graphics;
use crate::sdl2::rect::Rect;
use crate::sdl2::render::Texture;

pub struct Sprite<'a> {
    _sourceRect: Rect,
    _spriteSheet: &'a Texture<'a>,
    _x: i32,
    _y: i32,
}

impl<'a> Sprite<'a> {
    pub fn new(
        graphics: &'a Graphics,
        filepath: String,
        srcRect: Rect,
        width: i32,
        height: i32,
        x: i32,
        y: i32,
    ) -> Self {
        let spriteSheet = graphics.loadImage(&filepath).unwrap();

        let spriteSheet: Texture<'a> = graphics
            .texture_creator
            .create_texture_from_surface(spriteSheet)
            .unwrap();

        return Self {
            _sourceRect: srcRect,
            _spriteSheet: &spriteSheet,
            _x: x,
            _y: y,
        };
    }

    pub fn draw(&mut self,graphics:Graphics,x:i32,y:i32){
        let dstRect:Rect = Rect::new(x, y, self._sourceRect.w as u32, self._sourceRect.h as u32);

        graphics.blitSurface(self._spriteSheet,self._sourceRect,dstRect);
    }
}

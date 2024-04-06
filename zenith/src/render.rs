use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::sdl2_renderer;
use crate::{RenderingEngine2D, Screen};

pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y:i32) -> Self {
        Vec2 {
            x: x,
            y: y
        }
    }
}

pub struct VisualRect {
    pub location: Vec2,
    pub size: Vec2
}

impl VisualRect {
    pub fn new(location: Vec2, size: Vec2) -> Self {
        VisualRect {
            location: location,
            size: size
        }
    }
}

pub enum RenderingEnvironment {
    Sdl2(Sdl2Env),
}

pub struct Sdl2Env {
    pub canvas: Canvas<Window>,
}

#[allow(non_snake_case)]
pub fn new_2D_window(engine: RenderingEngine2D, screen: Screen) -> RenderingEnvironment {
    match engine {
        RenderingEngine2D::Sdl2 => {
            return RenderingEnvironment::Sdl2(sdl2_renderer::new_window(screen));
        }
    }
}




use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::sdl2_renderer;


use crate::{RenderingEngine2D, Screen};

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
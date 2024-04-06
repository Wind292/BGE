use sdl2::render::{self, Canvas};
use sdl2::video::Window;
use sdl2::Sdl;
use crate::sdl2_renderer::{self, sdl2_pressed_close};
use crate::{RenderingEngine2D, Screen};

#[derive(Clone)]
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
    pub sdl_context: Sdl
}

pub struct  Keys {
    A: bool,
    B: bool,
    C: bool,
    D: bool,
    E: bool,
    F: bool,
    G: bool,
    H: bool,
    I: bool,
    J: bool,
    K: bool,
    L: bool,
    M: bool,
    N: bool,
    O: bool,
    P: bool,
    Q: bool,
    R: bool,
    S: bool,
    T: bool,
    U: bool,
    V: bool,
    W: bool,
    X: bool,
    Y: bool,
    Z: bool,
    SPACE: bool,
    QUIT: bool,
    RSHIFT: bool,
    LSHIFT: bool,
    ESCAPE: bool,
    NUM_1: bool,
    NUM_2: bool,
    NUM_3: bool,
    NUM_4: bool,
    NUM_5: bool,
    NUM_6: bool,
    NUM_7: bool,
    NUM_8: bool,
    NUM_9: bool,
    NUM_0: bool,
}
impl Keys {
    pub fn new() -> Self {
        Keys {
            A: false,
            B: false,
            C: false,
            D: false,
            E: false,
            F: false,
            G: false,
            H: false,
            I: false,
            J: false,
            K: false,
            L: false,
            M: false,
            N: false,
            O: false,
            P: false,
            Q: false,
            R: false,
            S: false,
            T: false,
            U: false,
            V: false,
            W: false,
            X: false,
            Y: false,
            Z: false,
            SPACE: false,
            QUIT: false,
            RSHIFT: false,
            LSHIFT: false,
            ESCAPE: false,
            NUM_1: false,
            NUM_2: false,
            NUM_3: false,
            NUM_4: false,
            NUM_5: false,
            NUM_6: false,
            NUM_7: false,
            NUM_8: false,
            NUM_9: false,
            NUM_0: false, 
        }
    }
}


#[allow(non_snake_case)]
pub fn new_2D_window(engine: RenderingEngine2D, screen: Screen) -> RenderingEnvironment {
    match engine {
        RenderingEngine2D::Sdl2 => {
            return RenderingEnvironment::Sdl2(sdl2_renderer::new_window(screen));
        }
    }
}

pub fn draw_rect(rect: VisualRect, env: &mut RenderingEnvironment)  {
    match env {
        RenderingEnvironment::Sdl2(sld2_env) => sdl2_renderer::sdl2_draw_rect(sld2_env, rect)
    }
}

#[allow(irrefutable_let_patterns)]
pub fn pressed_close(engine: &RenderingEngine2D, rendering_environment: &RenderingEnvironment) -> bool {
    match engine {
        RenderingEngine2D::Sdl2 => {
            if let RenderingEnvironment::Sdl2(env) = rendering_environment {
                sdl2_pressed_close(env)
            }
            else {
                false
            }
        }
    }
}


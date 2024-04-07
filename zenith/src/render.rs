use crate::sdl2_renderer::{self};
use crate::{Instance2D, RenderingEngine2D, Screen};
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::Sdl;

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x: x, y: y }
    }
}

pub struct VisualRect {
    pub location: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl VisualRect {
    pub fn new(location: Vec2, size: Vec2, color: Color) -> Self {
        VisualRect {
            location: location,
            size: size,
            color: color,
        }
    }
}

pub enum RenderingEnvironment {
    Sdl2(Sdl2Env),
}

pub struct Sdl2Env {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Keys {
    pub A: bool,
    pub B: bool,
    pub C: bool,
    pub D: bool,
    pub E: bool,
    pub F: bool,
    pub G: bool,
    pub H: bool,
    pub I: bool,
    pub J: bool,
    pub K: bool,
    pub L: bool,
    pub M: bool,
    pub N: bool,
    pub O: bool,
    pub P: bool,
    pub Q: bool,
    pub R: bool,
    pub S: bool,
    pub T: bool,
    pub U: bool,
    pub V: bool,
    pub W: bool,
    pub X: bool,
    pub Y: bool,
    pub Z: bool,
    pub SPACE: bool,
    pub QUIT: bool,
    pub RSHIFT: bool,
    pub LSHIFT: bool,
    pub ESCAPE: bool,
    pub NUM_1: bool,
    pub NUM_2: bool,
    pub NUM_3: bool,
    pub NUM_4: bool,
    pub NUM_5: bool,
    pub NUM_6: bool,
    pub NUM_7: bool,
    pub NUM_8: bool,
    pub NUM_9: bool,
    pub NUM_0: bool,
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

impl Keys {
    pub fn all_pressed_str(&self) -> Vec<&str> {
        let mut pressed: Vec<&str> = vec![];
        if self.ESCAPE == true {
            pressed.push("escape")
        }
        if self.SPACE == true {
            pressed.push("space")
        }
        if self.LSHIFT == true {
            pressed.push("lshift")
        }
        if self.RSHIFT == true {
            pressed.push("rshift")
        }
        if self.A == true {
            pressed.push("a")
        }
        if self.B == true {
            pressed.push("b")
        }
        if self.C == true {
            pressed.push("c")
        }
        if self.D == true {
            pressed.push("d")
        }
        if self.E == true {
            pressed.push("e")
        }
        if self.F == true {
            pressed.push("f")
        }
        if self.G == true {
            pressed.push("g")
        }
        if self.H == true {
            pressed.push("h")
        }
        if self.I == true {
            pressed.push("i")
        }
        if self.J == true {
            pressed.push("j")
        }
        if self.K == true {
            pressed.push("k")
        }
        if self.L == true {
            pressed.push("l")
        }
        if self.M == true {
            pressed.push("m")
        }
        if self.N == true {
            pressed.push("n")
        }
        if self.O == true {
            pressed.push("o")
        }
        if self.P == true {
            pressed.push("p")
        }
        if self.Q == true {
            pressed.push("q")
        }
        if self.R == true {
            pressed.push("r")
        }
        if self.S == true {
            pressed.push("s")
        }
        if self.T == true {
            pressed.push("t")
        }
        if self.U == true {
            pressed.push("u")
        }
        if self.V == true {
            pressed.push("v")
        }
        if self.W == true {
            pressed.push("w")
        }
        if self.X == true {
            pressed.push("x")
        }
        if self.Y == true {
            pressed.push("y")
        }
        if self.Z == true {
            pressed.push("z")
        }
        pressed
    }
}

pub fn update_keystrokes(instance: &mut Instance2D) {
    match &instance.engine_settings.engine_env {
        RenderingEnvironment::Sdl2(sdl2_env) => {
            for event in sdl2_env.sdl_context.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } => instance.engine_settings.keys.QUIT = true,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        // Print the pressed key to the console
                        match key {
                            Keycode::Escape => instance.engine_settings.keys.ESCAPE = true,
                            Keycode::Space => instance.engine_settings.keys.SPACE = true,
                            Keycode::LShift => instance.engine_settings.keys.LSHIFT = true,
                            Keycode::RShift => instance.engine_settings.keys.RSHIFT = true,
                            Keycode::A => instance.engine_settings.keys.A = true,
                            Keycode::B => instance.engine_settings.keys.B = true,
                            Keycode::C => instance.engine_settings.keys.C = true,
                            Keycode::D => instance.engine_settings.keys.D = true,
                            Keycode::E => instance.engine_settings.keys.E = true,
                            Keycode::F => instance.engine_settings.keys.F = true,
                            Keycode::G => instance.engine_settings.keys.G = true,
                            Keycode::H => instance.engine_settings.keys.H = true,
                            Keycode::I => instance.engine_settings.keys.I = true,
                            Keycode::J => instance.engine_settings.keys.J = true,
                            Keycode::K => instance.engine_settings.keys.K = true,
                            Keycode::L => instance.engine_settings.keys.L = true,
                            Keycode::M => instance.engine_settings.keys.M = true,
                            Keycode::N => instance.engine_settings.keys.N = true,
                            Keycode::O => instance.engine_settings.keys.O = true,
                            Keycode::P => instance.engine_settings.keys.P = true,
                            Keycode::Q => instance.engine_settings.keys.Q = true,
                            Keycode::R => instance.engine_settings.keys.R = true,
                            Keycode::S => instance.engine_settings.keys.S = true,
                            Keycode::T => instance.engine_settings.keys.T = true,
                            Keycode::U => instance.engine_settings.keys.U = true,
                            Keycode::V => instance.engine_settings.keys.V = true,
                            Keycode::W => instance.engine_settings.keys.W = true,
                            Keycode::X => instance.engine_settings.keys.X = true,
                            Keycode::Y => instance.engine_settings.keys.Y = true,
                            Keycode::Z => instance.engine_settings.keys.Z = true,
                            _ => {}
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        // Print the pressed key to the console
                        match key {
                            Keycode::Escape => instance.engine_settings.keys.ESCAPE = false,
                            Keycode::Space => instance.engine_settings.keys.SPACE = false,
                            Keycode::LShift => instance.engine_settings.keys.LSHIFT = false,
                            Keycode::RShift => instance.engine_settings.keys.RSHIFT = false,
                            Keycode::A => instance.engine_settings.keys.A = false,
                            Keycode::B => instance.engine_settings.keys.B = false,
                            Keycode::C => instance.engine_settings.keys.C = false,
                            Keycode::D => instance.engine_settings.keys.D = false,
                            Keycode::E => instance.engine_settings.keys.E = false,
                            Keycode::F => instance.engine_settings.keys.F = false,
                            Keycode::G => instance.engine_settings.keys.G = false,
                            Keycode::H => instance.engine_settings.keys.H = false,
                            Keycode::I => instance.engine_settings.keys.I = false,
                            Keycode::J => instance.engine_settings.keys.J = false,
                            Keycode::K => instance.engine_settings.keys.K = false,
                            Keycode::L => instance.engine_settings.keys.L = false,
                            Keycode::M => instance.engine_settings.keys.M = false,
                            Keycode::N => instance.engine_settings.keys.N = false,
                            Keycode::O => instance.engine_settings.keys.O = false,
                            Keycode::P => instance.engine_settings.keys.P = false,
                            Keycode::Q => instance.engine_settings.keys.Q = false,
                            Keycode::R => instance.engine_settings.keys.R = false,
                            Keycode::S => instance.engine_settings.keys.S = false,
                            Keycode::T => instance.engine_settings.keys.T = false,
                            Keycode::U => instance.engine_settings.keys.U = false,
                            Keycode::V => instance.engine_settings.keys.V = false,
                            Keycode::W => instance.engine_settings.keys.W = false,
                            Keycode::X => instance.engine_settings.keys.X = false,
                            Keycode::Y => instance.engine_settings.keys.Y = false,
                            Keycode::Z => instance.engine_settings.keys.Z = false,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
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

pub fn draw_rect(rect: VisualRect, env: &mut RenderingEnvironment) {
    match env {
        RenderingEnvironment::Sdl2(sld2_env) => sdl2_renderer::sdl2_draw_rect(sld2_env, rect),
    }
}

pub fn update_display(env: &mut RenderingEnvironment) {
    match env {
        RenderingEnvironment::Sdl2(sld2_env) => sdl2_renderer::sdl2_update_display(sld2_env),
    }
}

use crate::render::Sdl2Env;
use crate::Screen;


pub fn new_window(screen: Screen) -> Sdl2Env{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(&screen.caption, screen.window_size.0, screen.window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    Sdl2Env {
        canvas: canvas,

    }
}
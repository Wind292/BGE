use crate::render::Sdl2Env;
use crate::render::VisualRect;
use crate::Screen;

use sdl2::event::Event;
use sdl2::rect::Rect;

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
        sdl_context: sdl_context
    }
}

pub fn sdl2_pressed_close(sdl2_env: &Sdl2Env) -> bool {
    if let Some(event) = sdl2_env.sdl_context.event_pump().unwrap().poll_event() {
        match event {
            Event::Quit {..} => {
                return true
            },
            _ => {}
        }
    }
    return false
}

pub fn sdl2_draw_rect(sdl2_env: &mut Sdl2Env, rect: VisualRect) {
    let width_result = rect.size.x.try_into();
    let width: u32 = match width_result {
        Ok(val) => val,
        Err(_) => {
            // Handle the error, for example:
            println!("Error: Width cannot be negitive - setting Width to 0");
            0 // Default value
        }
    };
    
    // Handling errors for converting rect.size.y to u32
    let length_result = rect.size.y.try_into();
    let length: u32 = match length_result {
        Ok(val) => val,
        Err(_) => {
            // Handle the error, for example:
            eprintln!("Error: Length cannot be negitive - setting Length to 0");
            0 // Default value
        }
    };


    let _ = sdl2_env.canvas.draw_rect(Rect::new(rect.location.x, rect.location.y, width, length));
}
use render::RenderingEnvironment;

mod render;
mod sdl2_renderer;

// !!EXAMPLE!!
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// !!  !!!  !!



pub struct Instance2D {
    pub screen: Screen,
    pub engine_settings: EngineSettings2D,
}

pub struct Screen {
    caption: String,
    framerate_cap: u32,
    window_size: (u32,u32),
}

pub struct EngineSettings2D {
    rendering_engine: RenderingEngine2D,
    use_delta_time: bool,
    engine_env: RenderingEnvironment
}

#[derive(Clone)]
pub enum RenderingEngine2D {
    Sdl2,
}

impl Instance2D {
    pub fn new() -> Self {
        Instance2D {
            // Default values
            screen: Screen::new(),
            engine_settings: EngineSettings2D::new(),
        }
    }
}

impl EngineSettings2D {
    pub fn new() -> Self {
        let engine = RenderingEngine2D::Sdl2;

        EngineSettings2D {
            // Default values
            rendering_engine: engine.clone(),
            engine_env: render::new_2D_window(engine, Screen::new()),
            use_delta_time: true
        }
        
    }
}


// Adding functionality to the Screen struct 
impl Screen {
    pub fn new() -> Self {
        Screen {
            // Default values
            caption: String::from("Zenith Game Window"),
            framerate_cap: 60,
            window_size: (600, 400)
        }
    }

    pub fn set_caption(&mut self, caption: &str) {
        self.caption = String::from(caption);
    }

    pub fn set_frame_rate_cap(&mut self, cap: u32) {
        self.framerate_cap = cap;
    }

    pub fn set_window_size(&mut self, size: (u32,u32)) {
        self.window_size = size;
    }


    pub fn get_caption(&self) -> &String {
        &self.caption
    }

    pub fn get_framerate_cap(&self) -> u32 {
        self.framerate_cap
    }

    pub fn get_window_size(&self) -> &(u32, u32) {
        &self.window_size
    }

}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2) ;
        assert_eq!(result, 4);
    }
}

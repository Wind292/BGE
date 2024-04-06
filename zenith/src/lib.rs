use std::vec;

use render::{pressed_close, RenderingEnvironment, Vec2};
use sdl2::{event::Event, libc::close};

mod eventloop;
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
    pub environment: Environment,
}

#[derive(Clone)]
pub struct Screen {
    caption: String,
    framerate_cap: u32,
    window_size: (u32,u32),
}

pub struct EngineSettings2D {
    rendering_engine: RenderingEngine2D,
    use_delta_time: bool,
    engine_env: RenderingEnvironment,
    is_running: bool,
}
#[derive(Clone)]
pub struct Environment {
    entities: Vec<Entity>
}
#[derive(Clone, Copy)]
pub enum RunEvent {
    Quit,
}

pub struct  RunEventList {
    events: Vec<RunEvent>
}

impl RunEventList {
    pub fn add_event(&mut self, event: RunEvent) {
        self.events.push(event);
    }
    pub fn single(event: RunEvent) -> Self {
        RunEventList {
            events: vec![event]
        }
    }
    pub fn empty() -> Self {
        RunEventList {
            events: vec![]
        }
    }
    pub fn merge(&mut self, other_list: &mut RunEventList) {
        self.events.append(&mut other_list.events)
    }
}

#[derive(Clone)]
pub struct Entity {
    location: Option<Vec2>,
    velocity: Option<Vec2>,
    size: Option<Vec2>,
    update_function: Option<fn(&Instance2D)-> RunEventList>,
    start_function: Option<fn(&Instance2D)-> RunEventList>
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
            environment: Environment::new()
        }
    }
    
    pub fn start(&mut self) {
        eventloop::eventloop(self)
    }

}

impl EngineSettings2D {
    pub fn new() -> Self {
        let engine = RenderingEngine2D::Sdl2;

        EngineSettings2D {
            // Default values
            rendering_engine: engine.clone(),
            engine_env: render::new_2D_window(engine, Screen::new()),
            use_delta_time: true,
            is_running: true
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

    pub fn set_framerate_cap(&mut self, cap: u32) {
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

impl Environment {
    pub fn new() -> Self {
        Environment {
            entities: get_builtin_entities()
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

fn get_builtin_entities() -> Vec<Entity> {
    let mut entities = vec![];

    entities.push(Entity {location: None, velocity: None, size: None, update_function: Some(close_window), start_function: None});

    entities
}

fn close_window(instance: &Instance2D) -> RunEventList {
    if pressed_close(&instance.engine_settings.rendering_engine, &instance.engine_settings.engine_env) {
        return RunEventList::single(RunEvent::Quit);
    }
    RunEventList::empty()
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

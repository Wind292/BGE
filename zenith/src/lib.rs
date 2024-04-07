use std::vec;

use render::{ Color, Keys, RenderingEnvironment, Vec2};

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
    window_size: (u32, u32),
}

pub struct EngineSettings2D {
    pub use_delta_time: bool,
    engine_env: RenderingEnvironment,
    pub is_running: bool,
    pub keys: Keys,
}
#[derive(Clone)]
pub struct Environment {
    entities: Vec<Entity>,
}
// #[derive(Clone, Copy)]
// pub enum RunEvent {
//     Quit,
// }

// pub struct  RunEventList {
//     events: Vec<RunEvent>
// }

// impl RunEventList {
//     pub fn add_event(&mut self, event: RunEvent) {
//         self.events.push(event);
//     }
//     pub fn single(event: RunEvent) -> Self {
//         RunEventList {
//             events: vec![event]
//         }
//     }
//     pub fn empty() -> Self {
//         RunEventList {
//             events: vec![]
//         }
//     }
//     pub fn merge(&mut self, other_list: &mut RunEventList) {
//         self.events.append(&mut other_list.events)
//     }
// }

struct EntityTag<T> {
    tag_name: String,
    tag_value: TagValue<T>,
}

enum TagValue<T> {
    Value(T),
}

#[derive(Clone)]
pub struct Entity {
    pub location: Option<Vec2>,
    pub velocity: Option<Vec2>,
    pub size: Option<Vec2>,
    pub update_function: Option<fn(&mut Instance2D)>,
    pub start_function: Option<fn(&mut Instance2D)>,
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            location: None,
            velocity: None,
            size: None,
            update_function: None,
            start_function: None,
        }
    }

    pub fn with_location(self, location: Vec2) -> Self {
        let mut x = self;
        x.location = Some(location);
        x
    }
    pub fn with_velocity(self, velocity: Vec2) -> Self {
        let mut x = self;
        x.velocity = Some(velocity);
        x
    }
    pub fn with_size(self, size: Vec2) -> Self {
        let mut x = self;
        x.size = Some(size);
        x
    }
    pub fn with_update_fn(self, update_fn: fn(&mut Instance2D)) -> Self {
        let mut x = self;
        x.update_function = Some(update_fn);
        x
    }
    pub fn with_start_fn(self, start_fn: fn(&mut Instance2D)) -> Self {
        let mut x = self;
        x.start_function = Some(start_fn);
        x
    }
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
            environment: Environment::new(),
        }
    }

    pub fn start(self) {
        eventloop::eventloop(self)
    }

    pub fn quit(&mut self) {
        self.engine_settings.is_running = false
    }

    pub fn get_pressed(&self) -> &Keys {
        &self.engine_settings.keys
    }

    pub fn update_display(&mut self) {
        
    }


}

impl EngineSettings2D {
    pub fn new() -> Self {
        let engine = RenderingEngine2D::Sdl2;

        EngineSettings2D {
            // Default values
            engine_env: render::new_2D_window(engine, Screen::new()),
            use_delta_time: true,
            is_running: true,
            keys: Keys::new(),
        }
    }

    pub fn update_display(&mut self) {
        render::update_display(&mut self.engine_env)
    }
}

// Adding functionality to the Screen struct

impl Screen {
    pub fn new() -> Self {
        Screen {
            // Default values
            caption: String::from("Zenith Game Window"),
            framerate_cap: 60,
            window_size: (600, 400),
        }
    }

    pub fn set_caption(&mut self, caption: &str) {
        self.caption = String::from(caption);
    }

    pub fn set_framerate_cap(&mut self, cap: u32) {
        self.framerate_cap = cap;
    }

    pub fn set_window_size(&mut self, size: (u32, u32)) {
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
            entities: get_builtin_entities(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

fn get_builtin_entities() -> Vec<Entity> {
    let mut entities = vec![];

    entities.push(Entity::new().with_update_fn(close_window_builtin));
    entities.push(Entity::new().with_update_fn(update_display_builtin));

    entities
}

fn close_window_builtin(instance: &mut Instance2D) {
    if instance.get_pressed().QUIT {
        instance.quit()
    }
}

fn update_display_builtin(instance: &mut Instance2D) {
    instance.engine_settings.update_display();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

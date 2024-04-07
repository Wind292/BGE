use render::{Color, Keys, RenderingEnvironment, Vec2};
use std::collections::HashMap;

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
    entities: Vec<Entity>, // Default Entity tag type is set to i32
    update_scripts: Vec<(String, fn(&mut Instance2D))>,
    start_scripts: Vec<(String, fn(&mut Instance2D))>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            entities: Vec::new(),
            update_scripts: get_builtin_update_functions(),
            start_scripts: Vec::new(),
        }
    }

    pub fn new_skeleton() -> Self {
        Environment {
            entities: Vec::new(),
            update_scripts: Vec::new(),
            start_scripts: Vec::new(),
        }
    }

    pub fn add_update_script(&mut self, name: &str, script: fn(&mut Instance2D)) {
        self.update_scripts.push((name.to_string(), script))
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn mut_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }
    pub fn list_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn print_all_entities(&self) {
        for ent in &self.entities {
            if let Some(name) = ent.get_tag("name") {
                println!("{:?} :", name)
            } else {
                println!("'Unnamed Entity' :")
            }

            ent.print_tags();
            print!("\n");
        }
    }

    pub fn print_all_scripts(&self) {
        println!("Start Scripts -");
        for script in &self.start_scripts {
            println!("    {}",script.0)
        }
        print!("\n");
        println!("Update Scripts -");
        for script in &self.update_scripts {
            println!("    {}",script.0)
        }
    }

    // Might be slow ngl
    pub fn overwrite(&mut self, name: &str, entity_to_change: &mut Entity) {
        for entity in &mut self.entities {
            if entity.get_name() == name {
                *entity = entity_to_change.clone(); // Assuming Entity implements Clone
                return;
            }
        }
        eprintln!("Cannot replace - No entity found with the name : {}", name)
    }
}

#[derive(Debug, Clone)]
pub enum TagValue {
    String(String),
    Int(i32),
    Float(f32),
    Double(f64),
    Color(Color),
    Vec2(Vec2),
}

#[derive(Clone)]
pub struct Entity {
    pub update_function: Option<fn(&mut Entity)>,
    pub start_function: Option<fn(&mut Entity)>,
    pub tags: HashMap<String, TagValue>,
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            update_function: None,
            start_function: None,
            tags: HashMap::new(),
        }
    }

    pub fn with_update_fn(self, update_fn: fn(&mut Entity)) -> Self {
        let mut x = self;
        x.update_function = Some(update_fn);
        x
    }

    pub fn with_start_fn(self, start_fn: fn(&mut Entity)) -> Self {
        let mut x = self;
        x.start_function = Some(start_fn);
        x
    }

    pub fn with_tag(self, tag_name: &str, tag_value: TagValue) -> Self {
        let mut x = self;
        x.tags.insert(tag_name.to_string(), tag_value);
        x
    }
    pub fn with_name_tag(self, name: &str) -> Self {
        let mut x = self;
        x.tags
            .insert("name".to_string(), TagValue::String(name.to_string()));
        x
    }

    pub fn get_tag(&self, tag_name: &str) -> Option<TagValue> {
        self.tags.get(tag_name).cloned()
    }

    pub fn get_name(&self) -> String {
        match self.tags.get("name") {
            Some(TagValue::String(a)) => a.to_string(),
            _ => "'Unnamed'".to_string(),
        }
    }

    pub fn print_tags(&self) {
        for tag in &self.tags {
            match tag {
                _ => println!("{} : {:?}", tag.0, tag.1),
            }
        }
    }
}

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

    pub fn new_skeleton() -> Self {
        Instance2D {
            // Default values
            screen: Screen::new(),
            engine_settings: EngineSettings2D::new(),
            environment: Environment::new_skeleton(),
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

    pub fn add_tag_handler() {}
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

fn get_builtin_update_functions() -> Vec<(String, fn(&mut Instance2D))> {
    let mut scripts: Vec<(String, fn(&mut Instance2D))> = Vec::new();

    scripts.push((
        "Close Window Function - BUILT-IN".to_string(),
        close_window_builtin,
    )); //"Close Window Function - BUILT-IN"
    scripts.push((
        "Update Display Function - BUILT-IN".to_string(),
        update_display_builtin,
    )); //Update Display Function - BUILT-IN

    scripts
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

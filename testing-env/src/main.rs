use zenith::*;

fn main() {
    let mut instance = Instance2D::new();

    let e = Entity::new().with_update_fn(up).with_tag("Test".to_string(), 12);
    
    instance.environment.add_entity(e);
    instance.start()
}

fn up(inst: &mut Instance2D) {
    if inst.get_pressed().ESCAPE {
        inst.quit()
    }
}